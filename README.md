# Brack (EN / [JA](./README_JA.md))

![](./brack-header.png)

[![Release](https://img.shields.io/github/v/release/brack-lang/brack.svg)](https://github.com/user/repository)
[![CI](https://github.com/brack-lang/brack/actions/workflows/ci.yml/badge.svg)](https://github.com/brack-lang/brack/actions/workflows/ci.yml)


> [!CAUTION]
> This project is experimental and just getting started. It is not a practical software. Your contribution would be very welcome!

Brack is expandable by WebAssembly the lightweight markup language.
It has the brackets-based simple syntax.

## Feature
- The three pairs of brackets based simple syntax
- Expandable the language by **WebAssembly**
- Choosable multiple backend languages
- AST based powerful macro system
- Provides plugin system, package manager, and language server

## Installation
Brack can be installed with some package managers.

### Cargo
```sh
cargo install --git https://github.com/brack-lang/brack brack
```

### Nix
```sh
nix profile install github:brack-lang/brack
```

## Build
Nix that enables Flakes is needed to build Brack.

```sh
nix build .
```

## Syntax
Brack defines three kinds of command-calling syntaxes.
They can be written as `[module.ident arg1, arg2, ..., argN]`, where `module` refers to a module name (which is defined once per plugin), `ident` refers to the command identifier, and `arg1, arg2, ..., argN` are the arguments.

`[]` is used for inline commands, while `{}` is used for block commands. Both are replaced with strings.
`<>` is used for macros, enabling the abstract syntax tree (AST) to transform into a different AST during compile time.

Below is a Brack document that is converted to HTML using the [std.html](https://github.com/brack-lang/std.html) plugin:

```brack
{std.* Hello, World!}
Hello, this is a document written using [std.* Brack]<std.^ a markup language extendable via WebAssembly>.
```

No commands for specific transformations are defined in the core Brack compiler.

## License
MIT OR Apache-2.0

