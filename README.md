# Noir Static Analyzer (PoC)

Noir Static Analyzer is a **proof-of-concept (PoC)** tool designed for **Noir**, a domain-specific language (DSL) for writing **zero-knowledge proofs**. Inspired by **Cargo Clippy** and **Cargo Check**, it provides static analysis for Noir programs, making it familiar to Rust developers. Noir's syntax closely resembles Rust, borrowing its **control flow, functions, and type system**.

## Features

- **Modular architecture**: Designed to support multiple lint rules.
- **AST-based analysis**: Currently, it uses Noir’s **Abstract Syntax Tree (AST)** for linting.
- **Example lint implemented**: `unused-function` detects unused private and `pub(crate)` functions.
- **Future potential**: Some lints might use **ACIR (Abstract Circuit Intermediate Representation)** for deeper analysis.

## Possible Future Enhancements

While this PoC focuses on AST-based linting, analyzing **ACIR** could enable:
- **Detecting unnecessary constraints** in circuits.
- **Optimizing witness assignments**.
- **Identifying redundant gates** in the proof system.

## Installation

You can install the analyzer using Cargo:
```sh
cargo install --git https://github.com/walnuthq/noir-static-analyzer
```

Alternatively, clone the repository and build it manually:
```sh
git clone https://github.com/walnuthq/noir-static-analyzer.git
cd noir-static-analyzer
cargo build --release
```

## Usage

To run the analyzer on a Noir project, use:
```sh
cargo run --release -- --manifest-path <path-to-Nargo.toml>
```
By default, it looks for `Nargo.toml` in the current directory.

## Example

Given the following Noir code:
```noir
fn private_fn_1() {}
fn private_fn_2() {}
pub(crate) fn crate_fn_1() {}
pub(crate) fn crate_fn_2() {}
pub fn public_fn_1() { private_fn_1() }
pub fn public_fn_2() { public_fn_1() }
pub fn public_fn_3() { crate_fn_1() }
```
The analyzer reports `private_fn_2` and `crate_fn_2` as unused.

## Video Demonstration

A short demo showcasing how the analyzer works is available:

[![Watch the Demo](docs/demo.mkv)](docs/demo.mkv)

## More Information
- **Noir AST** (used for analysis): [noirc_frontend AST](https://github.com/noir-lang/noir/tree/master/compiler/noirc_frontend/src/ast)
- **ACIR** (potential future analysis): [ACIR repository](https://github.com/noir-lang/noir/tree/master/acvm-repo)

## Contribution
This is an early PoC and welcomes feedback or contributions. Feel free to open issues or pull requests in the [GitHub repository](https://github.com/walnuthq/noir-static-analyzer).

