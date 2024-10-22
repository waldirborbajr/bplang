**WARNING** This project is in its very initial development stage, not all
features are implemented yet, usage API still subjected to change until `1.0.0`

# BP Language Transpiler

This project is a transpiler for a custom language called **BP**. The BP language is a simple, C-like language with variable declarations and print statements. The transpiler, written in **Rust**, converts BP code into C code, compiles it using GCC, and runs the resulting executable.

## BP Language Syntax

A basic BP program consists of variable declarations, assignments, and print statements. Here's an example of a BP source file (`main.bp`):

```bp
m variable01 = 01;
m variable02 = 02;
c variable03 = "BP Language";
show "The future language start here";
```

