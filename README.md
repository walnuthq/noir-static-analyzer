# Noir Static Analyzer

> [!WARNING]
> This project is under development and might contain bugs.

Noir Static Analyzer is a **proof-of-concept** tool designed for **Noir**, a domain-specific language (DSL) for writing **zero-knowledge proofs**. Inspired by **Cargo Clippy** and **Cargo Check**, it provides static analysis for Noir programs, making it familiar to Rust developers. Noir's syntax closely resembles Rust, borrowing its **control flow, functions, and type system**.

## Features

- **Modular architecture**: Designed to support multiple lint rules.
- **AST-based analysis**: Currently, it uses Noir’s **Abstract Syntax Tree (AST)** for linting.
- **Example lint implemented**: `unused-function` detects unused private and `pub(crate)` functions.
- **Future potential**: Some lints might use **ACIR (Abstract Circuit Intermediate Representation)** for deeper analysis.

## Possible Future Enhancements

Analyzing **ACIR** could enable:
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
The analyzer reports:
```text
Using manifest path: "Nargo.toml"
Workspace root: ""
Package: hello
Entry point: "src/main.nr"
warning: Function 'private_fn_2' is unused
  --> src/main.nr:2:19
 | fn private_fn_2() {}
                    ^

warning: Function 'crate_fn_2' is unused
  --> src/main.nr:4:28
 | pub(crate) fn crate_fn_2() {}
```
## Video Demonstration

A short demo showcasing how the analyzer works is available:

https://github.com/user-attachments/assets/c4ede200-7949-4581-9d8b-c72e65acac9e

## Roadmap

**Unused or Redundant Code Lints**
- [x] Unsued Function
- [ ] Unused Variable / Value
- [ ] Unused Import
- [ ] Duplicate or Redundant Constraint
- [ ] Redundant Control Flow

**ZK-Specific Lints**
- [ ] Unconstrained Variable
- [ ] Public Output Depending on Private Input
- [ ] Missing Range Checks on Integers
- [ ] Improper Use of Unconstrained Functions
- [ ] Ineffective Constraints or Always-True Assertions

**Style Lints**
- [ ] Naming Conventions
- [ ] Shadowing Variables
- [ ] Overly Complex Function
- [ ] Idiomatic Code Suggestions

**Performance Lints**
- [ ] Dead Stores and Unused Assignments
- [ ] Inefficient Looping Constructs
- [ ] Redundant Re-computation
- [ ] Use of Non-ideal Operations
- [ ] Large Constraints or Wide Integers Usage

**Correctness Lints**
- [ ] Unchecked Division or Modulus
- [ ] Missing Constraints
- [ ] Ignored Return Values
- [ ] Inconsistent Type Usage or Overflow Risk
- [ ] Constant or Unreachable Branch Conditions

## More Information
- **Noir AST** (used for analysis): [noirc_frontend AST](https://github.com/noir-lang/noir/tree/master/compiler/noirc_frontend/src/ast)
- **ACIR** (potential future analysis): [ACIR repository](https://github.com/noir-lang/noir/tree/master/acvm-repo)

## Contribution
Contributions are welcome! Feel free to open issues or pull requests in the [GitHub repository](https://github.com/walnuthq/noir-static-analyzer).

## Appendix A: Lint Descriptions

### Unused or Redundant Code Lints

1. **Unused Function**  
   Detect functions that are never called anywhere. This is already implemented in the Noir Analyzer (e.g. flagging a helper function that isn’t referenced) and is similar to Rust’s `dead_code` lint.  
   Removing unused functions reduces code size and audit surface. If the function is intended for future use or an external call, developers can annotate it (once Noir supports an attribute similar to `#[allow(dead_code)]`).

2. **Unused Variable/Value**  
   Warn if a local variable is never read after being assigned, or if a value is calculated and not used. This includes function parameters that are never used inside the function body.  
   Such code might be vestigial or a sign that something was forgotten. For example, an unused function argument might indicate an incomplete implementation.  
   This lint aligns with common practice in many languages (Rust compiler warns on unused variables unless prefixed with `_`). It helps declutter the code and catch potential mistakes.

3. **Unused Import**  
   If a module or package is imported but none of its definitions are used, the static analyzer should warn to remove it.  
   This is analogous to Rust’s unused `use` warnings and keeps the circuit code minimal. Removing unused imports can slightly reduce compile times and avoid confusion about what dependencies are actually needed.

4. **Duplicate or Redundant Constraint**  
   Identify if the exact same assertion or requirement is written twice, or if one logically subsumes another.  
   For instance:
   ```noir
   assert(x < 100);
   assert(x < 100); // duplicate
   ```
   Or:
   ```noir
   assert(x == y);
   assert(x - y == 0); // redundant
   ```
   While the constraint system will simply have a duplicate constraint (which doesn’t harm correctness), it’s inefficient. The lint can merge or flag duplicates.

5. **Redundant Control Flow**  
   Flag any `if`/`else` blocks or `match` arms that do nothing or are identical, as well as empty loops.  
   For example:
   ```noir
   if condition {
       do_something();
   } else {
       do_something(); // identical to above
   }
   ```
   The else branch repeats the same code, so the condition is irrelevant. Or:
   ```noir
   for i in 0..n {
       // empty loop body
   }
   ```
   
### ZK-Specific Lints

1. **Unconstrained Variable**
   The lint triggers if a private witness or intermediate variable does not appear in any constraints, equality, or output - the value is never uset to enforce anything.
2. **Public Output Depending on Private Input**
   Warn if private (witness) value flows directly into a public output or public-facing variable without any cryptographic transformation
3. **Missing Range Checks on Integers**
   If unconstraind *Field* is used for values that have an expected range, the circuit might accept invalid out-of-range values.
4. **Improper Use of Unconstrained Functions**
5. **Ineffective Constraints or Always-True Assertions**

### Style Lints
1. **Naming Conventions**
2. **Shadowing Variables**
3. **Overly Complex Function**
4. **Idiomatic Code Suggestions**

### Performance Lints
1. **Dead Stores and Unused Assignments**
2. **Inefficient Looping Constructs**
3. **Redundant Re-computation**
4. **Use of Non-ideal Operations**
5. **Large Constraints or Wide Integers Usage**

### Correctness Lints
1. **Unchecked Division or Modulus**
2. **Missing Constraints**
3. **Ignored Return Values**
4. **Inconsistent Type Usage or Overflow Risk**
5. **Constant or Unreachable Branch Conditions**
