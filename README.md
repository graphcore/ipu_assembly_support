# IPU Assembly Support

This extension provides basic support for writing assembly for Graphcore's IPU chips.

## Features

* Basic syntax highlighting.
* Completion for instruction mnemonics and most register names.
* Definitions on hover for instructions and most registers.
* Signature helper for instructions.
* Go-to definition for `#define`s, `label:`s and `.macro`s in the same file.

## Limitations

* It uses a Tree-Sitter based parser which is not 100% accurate, therefore some things may not work perfectly.
* The `isa_doc_generated` Markdown content is automatically converted from RestructuredText to Markdown. The conversion is imperfect so there are some remnants of RST syntax in the Markdown text.
* It does not follow `#include`s at all to find definitions.

## Extension Settings

There are no configurable settings.

## License

The Graphcore logo (`images/icon.png` and `images/icon.svg`) is a registered trademark and cannot be used without written authorization from Graphcore. The ISA documentation (everything under `isa_doc_generated` and `isa_doc_manual`) is subject to Graphcore's copyrights, and cannot be reproduced, copied or distributed except with Graphcore's prior written authorization.

Everything else in this repository is licensed under the MIT license (see [`LICENSE.md`](https://github.com/graphcore/ipu_assembly_support/blob/master/LICENSE.md)). Licenses for dependencies are listed in [`THIRD_PARTY_LICENSES_NPM.txt`](https://github.com/graphcore/ipu_assembly_support/blob/master/THIRD_PARTY_LICENSES_NPM.txt) and [`THIRD_PARTY_LICENSES_RUST.html`](https://github.com/graphcore/ipu_assembly_support/blob/master/THIRD_PARTY_LICENSES_RUST.html).

## Development

To build this you need:

* Rust
* NPM
* [Tree-Sitter](https://tree-sitter.github.io/tree-sitter/creating-parsers#installation)

Then run:

```
./make release
```

To test it open this directory in VSCode and run it (F5 or Debug->Start Debugging).

To build a VSIX package (VSCode extension package), run `./make package`.

## How it works

The VSCode extension is contained in `client/extension.ts`. When activated it runs the Rust language server. The source code for that is in `server`. Currently it is compiled natively for Linux and Mac x86_64, so the extension will only work on those systems.

The server uses Tree-Sitter to parse the assembly files, using a grammar that's a sort of mix of the upstream Tree-Sitter C grammar (for the C preprocessor) and what I could make out of the LLVM assembly parser. Assembly is not really a real format, so there's no actual grammar, and combined with the C preprocessor it means this is a best effort.

The ISA details are stored in some Markdown files. These are transformed into a generated Rust file which is compiled into the server. The idea was that the Markdown files could be hand edited to improve them.

## TODO

### Wasm

To avoid having to cross-compile the language server and include multiple copies was can compile it to WASM and run it via Node. In order to do so we need to target the WASI platform, which is basically "POSIX for WASM". It can be compiled like this:

1. Download [wasi-sysroot and wasi-sdk](https://github.com/WebAssembly/wasi-sdk/releases) and unzip them somwhere.
2. `brew install llvm` to get the "real" LLVM (not Apple's version).
3. `export PATH=/path/to/wasi-sdk-99.9/bin:/usr/local/opt/llvm/bin:$PATH`
4. `export CFLAGS=--sysroot=/path/to/wasi-sysroot`
5. Change the `cargo build` commands to use `--target wasm32-wasi`.

This should give an `ipu_assembly_server.wasm` output file in the target directory somewhere. It's approximately 6 MB for the release version.

In order to execute it you need a WASM runtime. Options include Wasmtime and Wasmer, but they're both pretty big to download and then we'd have to download them for every platform. A better option is to use [Node's built in support](https://nodejs.org/api/wasi.html). Unfortunately it is still experimental and the Node version that VSCode uses (via Electron) is about a year old so we may have to wait a few years for this.

### `#include` support

Currently this only includes support for go-to-definition within the same file. It does not follow `#include`s. I think the only realistic way to do that is to have the user manually provide include paths via settings.

### Auto-nop insertion feedback

Nops are automatically inserted to align instruction bundles. We should be able to figure out when that happens and show the user. That would be much less tedious than counting instructions by hand, which is what I did.

E.g. search for `rpt alignment` in `Clamp.S` to see an example of why you'd want to do this. Probably the simplest thing is to add a warning diagnostic for bundles that are not aligned already.
