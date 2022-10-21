use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};

// Given a DirEntry, read it and return (filename, contents) iff it is a markdown file.
fn read_entry_markdown(entry: io::Result<fs::DirEntry>) -> Result<Option<(String, String)>> {
    let entry = entry?;
    let file_type = entry.file_type()?;
    let file_name = entry.file_name();
    let file_name = file_name.to_str().ok_or(anyhow!("Non-UTF8 filename"))?;
    if file_type.is_file() && file_name.ends_with(".md") {
        Ok(Some((
            file_name.trim_end_matches(".md").to_owned(),
            fs::read_to_string(entry.path())?,
        )))
    } else {
        Ok(None)
    }
}

// Read all of the markdown files in a directory.
fn read_markdown_files(dir: &Path) -> Result<BTreeMap<String, String>> {
    fs::read_dir(dir)?
        .filter_map(|entry| read_entry_markdown(entry).transpose())
        .collect()
}

struct CompletionDef {
    detail: String,
    documentation: String,
}

fn parse_completion_file(file: &str) -> Result<CompletionDef> {
    // The first line is the detail, and must start with `# `. Then there must
    // be a blank line, then the rest is the documentation.

    let mut iter = file.lines();

    let first_line = iter.next();
    let blank_line = iter.next();
    let third_line = iter.next();

    if let Some(first_line) = first_line {
        if !first_line.starts_with("# ") {
            bail!("First line must start with `# `");
        }
        let detail = &first_line[2..];

        if let Some(blank_line) = blank_line {
            if !blank_line.is_empty() {
                bail!("Second line must be empty");
            }
        }

        let documentation = if let Some(third_line) = third_line {
            let diff = third_line.as_ptr() as usize - file.as_ptr() as usize;
            &file[diff..]
        } else {
            ""
        };

        return Ok(CompletionDef {
            detail: detail.to_owned(),
            documentation: documentation.to_owned(),
        });
    }

    bail!("Empty file");
}

struct SignatureDef {
    label: String,
    documentation: String,
    parameters: Vec<SignatureParam>,
}

struct SignatureParam {
    label: String,
    documentation: String,
}

fn parse_signature_file(file: &str) -> Result<Vec<SignatureDef>> {
    // The H1 header is the label, and then the content is the documentation.
    // Then there is an H2 header and content for each parameter label /
    // documentation.
    // Then the whole thing is repeated for each signature.

    let mut defs: Vec<SignatureDef> = Vec::new();

    for line in file.lines() {
        if let Some(line_stripped) = line.strip_prefix("## ") {
            // Add a parameter
            if let Some(last) = defs.last_mut() {
                last.parameters.push(SignatureParam {
                    label: line_stripped.trim().to_owned(),
                    documentation: String::new(),
                });
            } else {
                bail!("## seen before # in signature file");
            }
        } else if let Some(line_stripped) = line.strip_prefix("# ") {
            defs.push(SignatureDef {
                label: line_stripped.trim().to_owned(),
                documentation: String::new(),
                parameters: Vec::new(),
            });
        } else {
            let is_space = line.trim().is_empty();

            if let Some(last_def) = defs.last_mut() {
                if let Some(last_param) = last_def.parameters.last_mut() {
                    // Append to the last parameter's doc unless it is preceeding whitespace.
                    if !last_param.documentation.is_empty() || !is_space {
                        last_param.documentation.push_str(line);
                        last_param.documentation.push('\n');
                    }
                } else {
                    // Append to the def's documentation unless this is preceeding whitespace.
                    if !last_def.documentation.is_empty() || !is_space {
                        last_def.documentation.push_str(line);
                        last_def.documentation.push('\n');
                    }
                }
            } else {
                bail!("Text seen before # in signature file");
            }
        }
    }

    Ok(defs)
}

/// This function reads a directory containing a load of markdown files
/// and converts them into a single Rust file.
pub fn md_to_rs(in_dirs: &[PathBuf], out_file: &Path) -> Result<()> {
    // Read files from all docs dirs. Later ones win if there are duplicates.
    let mut hover_defs = BTreeMap::<String, String>::new();
    let mut completion_defs = BTreeMap::<String, String>::new();
    let mut signature_defs = BTreeMap::<String, String>::new();
    for in_dir in in_dirs {
        hover_defs.append(&mut read_markdown_files(&in_dir.join("hover"))?);
        completion_defs.append(&mut read_markdown_files(&in_dir.join("completion"))?);
        signature_defs.append(&mut read_markdown_files(&in_dir.join("signature"))?);
    }

    // Parse completion and signatures. Hovers don't need to be parsed.
    let completion_defs: BTreeMap<String, CompletionDef> = completion_defs
        .into_iter()
        .map(|(k, v)| parse_completion_file(&v).map(|def| (k, def)))
        .collect::<Result<_>>()?;

    let signature_defs: BTreeMap<String, Vec<SignatureDef>> = signature_defs
        .into_iter()
        .map(|(k, v)| parse_signature_file(&v).map(|def| (k, def)))
        .collect::<Result<_>>()?;

    // Write it all to files.
    eprintln!("Opening output file");

    let file = std::fs::File::create(out_file)?;

    let mut writer = io::BufWriter::new(file);

    writeln!(
        &mut writer,
        "
// This file was automatically @generated by `isa`. Regenerate e.g. by:
// cd isa && cargo run -- md_to_rs ../doc ../server/src/generated.rs
"
    )?;

    write_hover_defs(&mut writer, &hover_defs)?;
    write_completion_defs(&mut writer, &completion_defs)?;
    write_signature_defs(&mut writer, &signature_defs)?;

    eprintln!("Done");
    Ok(())
}

fn write_hover_defs(writer: &mut impl io::Write, defs: &BTreeMap<String, String>) -> Result<()> {
    let mut map = phf_codegen::Map::<&str>::new();
    for (key, value) in defs {
        map.entry(key, &format!("{:?}", value));
    }
    writeln!(
        writer,
        "pub static HOVERS: phf::Map<&'static str, &'static str> = {};",
        map.build()
    )?;
    Ok(())
}

fn write_completion_defs(
    writer: &mut impl io::Write,
    defs: &BTreeMap<String, CompletionDef>,
) -> Result<()> {
    // Slightly over-optimisation: precalculate the range of completions for every
    // input that returns results and store that in a PHF map.

    // 0: a
    // 1: b
    // 2: baaa
    // 3: bbb
    // 4: c

    // a -> [0, 1)
    // b -> [1, 4)
    // ba -> [2, 3)
    // baa -> [2, 3)
    // baaa -> [2, 3)
    // bb -> [3, 4)
    // bbb -> [3, 4)
    // c -> [4, 5)

    // The algorithm is simple:
    //
    // 1. For each item, find the suffix that doesn't match the previous one, e.g.
    //    for line 3 it is 'bb'.
    // 2. Loop through each letter in that suffix.
    // 3. Find the last one that matches. For each of those the range is from the
    //    current entry to the last one that matches.

    let mut map = phf_codegen::Map::<&str>::new();

    let defs_vec: Vec<(&String, &CompletionDef)> = defs.iter().collect();

    let mut prev = "";

    for (i, def) in defs_vec.iter().enumerate() {
        let key = def.0;

        let key_iter = key.char_indices();
        let mut prev_iter = prev.chars();

        let mut matching = true;

        for (ci, c) in key_iter {
            matching = matching && matches!(prev_iter.next(), Some(p) if p == c);

            if !matching {
                // TODO: This assumes single-byte characters.
                let prefix = &key[0..=ci];
                // Find the next entry that doesn't start with prefix. Just
                // do linear search for now.
                let num_matches = defs_vec
                    .iter()
                    .skip(i)
                    .position(|def| !def.0.starts_with(prefix));
                let end = num_matches.map(|n| n + i).unwrap_or(defs_vec.len());
                map.entry(prefix, &format!("{}..{}", i, end));
            }
        }

        prev = def.0;
    }

    writeln!(
        writer,
        "
pub struct CompletionDef<'a> {{
    pub label: &'a str,
    pub detail: &'a str,
    pub documentation: &'a str,
}}
"
    )?;

    writeln!(writer, "pub const COMPLETIONS: &[CompletionDef] = &[")?;
    for def in &defs_vec {
        writeln!(
            writer,
            "  CompletionDef {{ label: {:?}, detail: {:?}, documentation: {:?} }},",
            def.0, def.1.detail, def.1.documentation
        )?;
    }
    writeln!(writer, "];")?;

    writeln!(
        writer,
        "pub static COMPLETION_RANGES: phf::Map<&'static str, std::ops::Range<usize>> = {};",
        map.build()
    )?;

    Ok(())
}

fn write_signature_defs(
    writer: &mut impl io::Write,
    defs: &BTreeMap<String, Vec<SignatureDef>>,
) -> Result<()> {
    writeln!(
        writer,
        "
pub struct SignatureDef<'a> {{
    pub label: &'a str,
    pub documentation: &'a str,
    pub parameters: &'a [SignatureParam<'a>],
}}

pub struct SignatureParam<'a> {{
    pub label: &'a str,
    pub documentation: &'a str,
}}
"
    )?;

    let mut map = phf_codegen::Map::<&str>::new();
    for (key, sigs) in defs {
        // Quote it. For whatever reason phf_codegen doesn't use proper types here.
        let mut v = String::new();
        write!(&mut v, "&[")?;
        for sig in sigs {
            write!(
                &mut v,
                "SignatureDef {{ label: {:?}, documentation: {:?}, parameters: &[",
                sig.label, sig.documentation
            )?;
            for param in &sig.parameters {
                write!(
                    &mut v,
                    "SignatureParam {{ label: {:?}, documentation: {:?} }}, ",
                    param.label, param.documentation
                )?;
            }
            write!(&mut v, "] }}, ")?;
        }
        write!(&mut v, "]")?;

        map.entry(key, &v);
    }
    writeln!(
        writer,
        "pub static SIGNATURES: phf::Map<&'static str, &'static [SignatureDef<'static>]> = {};",
        map.build()
    )?;

    Ok(())
}
