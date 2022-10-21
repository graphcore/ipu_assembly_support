mod md_to_rs;

use anyhow::Result;
use argh::FromArgs;
use md_to_rs::md_to_rs;
use std::path::PathBuf;

#[derive(FromArgs, PartialEq, Debug)]
/// Convert the markdown to a generated Rust file which is compiled into
/// the language server.
struct Opts {
    #[argh(positional)]
    /// the output Rust file
    out_file: PathBuf,

    #[argh(positional)]
    /// the input markdown directories
    in_dirs: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let opts: Opts = argh::from_env();

    md_to_rs(&opts.in_dirs, &opts.out_file)?;
    Ok(())
}
