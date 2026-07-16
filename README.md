<p align="center">
  <img width="256" height="256" src="./assets/logo.png" alt="BP Language Transpiler logo" />
</p>

<h1 align="center">BP Language Transpiler</h1>

<p align="center">
  A simple transpiler and interactive REPL for the experimental <strong>BP</strong> language, written in <strong>Rust</strong>.
</p>

**WARNING** This project is in an early development stage. Parts of the syntax, behavior, internal architecture, and CLI workflow may still change before `1.0.0`. A good README should explain purpose, setup, and usage clearly, with structured sections and runnable examples, which is the approach used here. [web:2]

# About

The **BP Language Transpiler** is a small experimental compiler front-end for a custom language called **BP**. It reads BP source code, tokenizes the input, parses it into an AST, transpiles the result to **C**, compiles the generated C file using **GCC**, and then executes the resulting binary.

In addition to file-based execution, the project now includes an interactive **REPL** mode. The REPL starts automatically when no `main.bp` file is found, allowing you to test declarations and `show` statements directly in the terminal. Interactive REPL tools commonly document prompt behavior and exit commands explicitly, which is reflected in the usage section below. [web:6][web:11][web:13]

# Features

- BP source tokenization.
- Simple parser that builds an AST from BP tokens.
- Transpilation from BP to C.
- Native compilation through `gcc`.
- Automatic execution of the compiled program.
- Interactive **REPL** mode when `main.bp` is not present.
- Support for numeric and string variable declarations.
- Support for `show` statements with string literals.
- Minimal and readable codebase for language experimentation.

# Current Status

This repository is intended for learning, experimentation, and iterative language design. The current implementation already covers the basic compilation pipeline:

- Lexing / tokenization.
- Parsing into AST nodes.
- C code generation.
- Native compilation and execution.
- Interactive prompt mode.

The language and runtime are still intentionally small. Some planned features are not implemented yet, such as richer expressions, variable printing, arithmetic, type checking, and stronger error reporting.

# Project Structure

A typical repository layout for this project may look like this:

```text
.
├── assets/
│   └── logo.png
├── main.bp
├── src/
│   └── main.rs
├── Cargo.toml
└── README.md
```

If the executable finds a `main.bp` file in the working directory, it runs in **file mode**. If `main.bp` does not exist, it falls back to **REPL mode**.

# How It Works

The project follows a straightforward compilation pipeline:

1. Read BP source code from `main.bp` or from user input in the REPL.
2. Convert the source into a list of tokens.
3. Parse the token list into AST nodes.
4. Generate equivalent C code.
5. Save the generated output to `main.c`.
6. Compile `main.c` with `gcc`.
7. Execute the generated binary.

This structure matches the common README recommendation of explaining not only what the project does, but also how a user can quickly understand and run it. [web:2]

# BP Language Syntax

The BP language is intentionally small and C-like. At the moment, it supports:

- Numeric variable declarations with `m`.
- String variable declarations with `c`.
- String output with `show`.

## Example

A basic BP source file (`main.bp`) looks like this:

```bp
m variable01 = 01;
m variable02 = 02;
c variable03 = "BP Language";
show "The future language start here";
```

## Supported statements

### Numeric declaration

```bp
m age = 25;
```

### String declaration

```bp
c name = "BP";
```

### Print string literal

```bp
show "Hello from BP";
```

# Language Rules

Based on the current implementation, these are the practical rules of the language:

- Variable declarations use `m` for numbers.
- Variable declarations use `c` for strings.
- `show` currently accepts a string literal.
- Statements are terminated with `;`.
- Numeric literals are parsed as integers.
- String literals must be wrapped in double quotes.
- Identifiers are alphabetic in the current tokenizer.
- Whitespace is ignored outside string literals.

## Important limitations

At the moment, the parser and tokenizer are intentionally minimal, so some limitations apply:

- Identifiers currently accept only alphabetic characters in the tokenizer, which means names like `variable01` from the example are not fully aligned with the current lexer behavior.
- `show` only handles string literals for now.
- Variables are stored in the REPL, but variable retrieval and interpolation are not yet implemented.
- The parser has basic error messages and limited recovery behavior.
- File mode expects a file named exactly `main.bp`.

# REPL

The project now includes an interactive **REPL** (Read-Eval-Print Loop). REPL documentation usually benefits from clearly describing the prompt, exit behavior, and interaction model, so those details are included here. [web:6][web:11][web:13]

## When REPL mode starts

The REPL starts automatically when the executable does **not** find a `main.bp` file in the current directory.

In other words:

- `main.bp` exists -> file mode.
- `main.bp` does not exist -> REPL mode.

## REPL behavior

When REPL mode starts, the program shows a prompt like this:

```text
BP Interactive Prompt (type 'exit' or Ctrl+C to quit)
> 
```

You can then enter BP statements one line at a time.

## Supported REPL actions

- Declare numeric variables.
- Declare string variables.
- Execute `show` with string literals.
- Exit with `exit`.
- Exit with `quit`.
- Interrupt with `Ctrl+C`.

Both `exit` and `quit` are widely documented conventions in interactive shells and REPL-style tools, and the current implementation follows that familiar pattern. [web:13][web:14]

## REPL examples

### Show text

```text
> show "Hello from REPL";
Hello from REPL
```

### Declare a numeric variable

```text
> m age = 42;
age = 42
```

### Declare a string variable

```text
> c language = "BP";
language = "BP"
```

### Exit the REPL

```text
> exit
```

## REPL notes

The REPL currently parses each entered line and executes supported nodes immediately. It also stores declared variables internally in a `HashMap`, which prepares the project for future features such as variable lookup, expression evaluation, and variable-based output.

# File Mode

When `main.bp` is present, the program runs in file mode.

## File mode workflow

1. Open `main.bp`.
2. Read the source code into memory.
3. Tokenize and parse the source.
4. Generate `main.c`.
5. Compile the generated code with `gcc`.
6. Run the compiled executable.

## Expected generated output

For a simple BP program like this:

```bp
m age = 25;
c title = "BP";
show "Hello";
```

The generated C code will follow the same general structure:

```c
#include <stdio.h>

int main() {
    int age = 25;
    char title[] = "BP";
    printf("Hello\n");
    return 0;
}
```

# Installation

## Requirements

You need the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [GCC](https://gcc.gnu.org/)

A strong README should include installation instructions and copy-paste-ready commands so users can get started quickly. [web:2]

## Clone the repository

```bash
git clone <your-repository-url>
cd <your-repository-folder>
```

## Build the project

```bash
cargo build
```

## Run the project

```bash
cargo run
```

For an optimized build:

```bash
cargo run --release
```

# Usage

## Run in file mode

Create a `main.bp` file in the project root:

```bp
m year = 2026;
c project = "BP Language Transpiler";
show "Running from file mode";
```

Then run:

```bash
cargo run
```

The expected behavior is:

- The BP source is parsed.
- A `main.c` file is generated.
- GCC compiles the file.
- The resulting executable runs.

## Run in REPL mode

Make sure there is **no** `main.bp` file in the current directory, then run:

```bash
cargo run
```

The program will start the interactive prompt instead of file execution.

# Supported AST Nodes

The current AST includes:

- `VariableDeclaration`
- `StringLiteral`
- `NumberLiteral`
- `Show`

This small AST is enough to support the current language subset while keeping the implementation approachable for compiler and interpreter study.

# Token Types

The tokenizer currently recognizes:

- `Keyword(String)`
- `Identifier(String)`
- `Number(i32)`
- `StringLiteral(String)`
- `Equals`
- `Semicolon`
- `EndOfFile`

These token categories are enough to model the language in its current form, though they will likely expand as the language grows.

# Example Session

## File mode example

### `main.bp`

```bp
m counter = 10;
c app = "BP";
show "Program started";
```

### Run

```bash
cargo run
```

### Possible output

```text
C code generated → main.c
Running program...
Program started
```

## REPL example

```text
BP Interactive Prompt (type 'exit' or Ctrl+C to quit)
> c tool = "BP";
tool = "BP"
> m version = 1;
version = 1
> show "Interactive mode";
Interactive mode
> quit
```

# Roadmap

Below are reasonable next steps for the project:

- Support identifiers with numbers and underscores.
- Support reassignment statements.
- Support `show` for variables.
- Support binary expressions.
- Add arithmetic operations.
- Add comments in BP source files.
- Improve parser diagnostics and syntax errors.
- Add better compile-time and runtime error reporting.
- Add tests for tokenizer, parser, and transpiler.
- Add CLI flags for choosing input and output files.
- Extend the REPL with command history and multiline input.

Interactive REPLs often evolve with features such as history, clear commands, configurable prompts, and better session handling, which makes these future improvements especially natural for this project. [web:11][web:12][web:13]

# Known Issues

- The current tokenizer only collects alphabetic characters for identifiers.
- Example identifiers like `variable01` are not fully compatible with the current lexer logic.
- Parsing stops early in some semicolon-related flows because of the current parser structure.
- `show` does not yet print variables.
- REPL variables are stored but not yet reused in expressions or output.
- Error handling is still basic and developer-oriented.

# Development Notes

This project is a good base for experimenting with compiler construction concepts in Rust:

- Lexer design.
- Recursive or index-based parsing.
- AST representation.
- Source-to-source transpilation.
- Native code generation through an intermediate language.
- Interactive execution models with a REPL.

Because the codebase is compact, it is suitable for learning, prototyping, and incremental refactoring.

# References

The project is conceptually related to educational resources on interpreters, parsers, and AST tooling.

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [AST Explorer](https://astexplorer.net/)

General README guidance also recommends covering project purpose, installation, usage, examples, and references in a clear structure, which this document follows. [web:2]

# Suggested README Badges

If you want to extend the repository presentation later, you can add badges such as:

```md



```

README best-practice guidance explicitly mentions badges as a common and useful enhancement for project front pages. [web:2]

# License

Choose the license that matches your intended use. A common choice for small experimental language tools is the MIT License.

Example:

```text
MIT License
```

README guidance commonly recommends including a license section so users know how the code can be used or modified. [web:2]

# Contributing

Contributions are welcome while the language is still evolving.

Possible ways to contribute:

- Fix tokenizer and parser edge cases.
- Improve transpilation output.
- Expand the REPL.
- Add tests.
- Improve documentation.
- Propose new BP syntax features.

## Suggested contribution flow

```bash
git checkout -b feature/my-change
cargo fmt
cargo test
git commit -m "Add my change"
git push origin feature/my-change
```

# Author

Created for experimentation with a custom language pipeline in **Rust**, combining transpilation, native compilation, and an interactive REPL workflow.

# Full Example README Snippet for Repository Root

Below is a compact repository-facing version you can place directly at the root if you want a shorter landing section before the detailed documentation:

```md
<p align="center">
  <img width="256" height="256" src="./assets/logo.png" alt="BP Language Transpiler logo" />
</p>

<h1 align="center">BP Language Transpiler</h1>

**WARNING** This project is in its very initial development stage, not all features are implemented yet, and the usage API is still subject to change until `1.0.0`.

A simple transpiler and REPL for the experimental **BP** language, written in **Rust**.

## Features

- Tokenizes BP source code
- Parses BP into an AST
- Transpiles BP to C
- Compiles generated C with GCC
- Runs the compiled executable
- Includes an interactive **REPL**
- Supports string and numeric declarations
- Supports `show` with string literals

## Usage

### File mode

Create `main.bp`:

```bp
m number = 10;
c name = "BP";
show "Hello from file mode";
```

Run:

```bash
cargo run
```

### REPL mode

Remove `main.bp` and run:

```bash
cargo run
```

Then use:

```text
> c lang = "BP";
lang = "BP"
> show "hello";
hello
> exit
```

## References

- [Crafting Interpreters](https://craftinginterpreters.com/)
- [AST Explorer](https://astexplorer.net/)
```
