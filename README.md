# Chimpanzee

## What is the Monkey language?

The Monkey language is a language created by Thorsten Ball for his book [Writing an Interpreter in Go](https://interpreterbook.com/). It is a dynamically typed language with C-like syntax. It supports integers, booleans, strings, arrays, hashes, and functions. It also has first-class functions, closures, and lexical scope.

## Chimpazee

Chimpazee is an implementation of the Monkey language in Rust. It is based on the books [Writing an Interpreter in Go](https://interpreterbook.com/) and [Writing a Compiler in Go](https://compilerbook.com/).

This implementation is still in development. For now an interpreter and a compiler are fully implemented, allowing to run a REPL and to run Monkey files (`.monkey` extension).
There are some issues that I want to fix before I can call this implementation complete.

### REPL

To start the REPL, run the following command:

```bash
monkey
```

### File interpreter

To run a Monkey file, run the following command:

```bash
monkey <path-to-file>
```

### Other modes

You can also test the compiler, parser and lexer in the same way, adding the following flag after the path to the file:

```bash
monkey --mode <mode>
```

Where `<mode>` can be `compiler`, `parser`, `lexer` or `interpreter`.

Example:

```bash
monkey <path-to-file> --mode compiler
```

### Formatter

A monkey formatter is also available, with the binary `monkeyfmt`. I will format any correct piece of monkey code.
To use it you only need to run the following command:

```bash
monkeyfmt <path-to-file>
```

Adding the `-r` flag after the file name will replace the contents of the file with the
formatted code. If the flag is not activated, the formatted code will be printed to
`stdout`.

### Help

To see the help, run the following command:

```bash
monkey --help
```

## Installation

### Crates.io

`Chimpanzee` is available as a cargo crate, which means that you can install it
by simple using:

```bash
cargo install chimpanzee
```

### From source

To install it from source you bust clone the repo. Once you have clone it you build the project

```bash
cargo build --release
```

> This step can take some time, the expected time is less that 2 minutes, but it can be even longer.

In the directory `target/directory` the two executables will be now available: `monkey` and `monkeyfmt`.

## Monkey language

Information about the monkey language is available in the [MONKEY file](docs/MONKEY.md).
