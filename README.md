# 🐦‍⬛ The Brack Markup Language

![](./assets/brack-header.png)

<div align="center">

**English** | [日本語](./README_JA.md)

[![Release](https://img.shields.io/github/v/release/brack-lang/brack.svg)](https://github.com/brack-lang/brack/tree/main)
[![Pre-Release](https://img.shields.io/github/v/release/brack-lang/brack.svg?include_prereleases&label=prerelease)](https://github.com/user/repository)
[![CI](https://github.com/brack-lang/brack/actions/workflows/ci.yml/badge.svg)](https://github.com/brack-lang/brack/actions/workflows/ci.yml)

[![Discord Invite Budge](https://dcbadge.limes.pink/api/server/cH94kqUMYH?style=flat)](https://discord.gg/cH94kqUMYH)
[![X Following Budge](https://img.shields.io/twitter/follow/:bracklanguage)](https://twitter.com/intent/user?screen_name=bracklanguage)

</div>

Brack is a highly extensible markup language with a simple syntax.
It has a plug-in system using WebAssembly binary, so you can add commands to this language using your familiar programming language[^1].

[^1]: However, the plug-in host language must have a compilation target to WebAssembly, and it must be implemented [Extism](https://extism.org) PDK (Plug-in Development Kit). As of 16 Octorbar 2024, the programming languages that are provided Extism PDK is 8 types, are Rust, JavaScript, Go, Haskell, AssemblyScript, C, Zig, and .NET. Also, from v0.2.0 we only provide Brack PDK for Rust. Contributions implementing Extism or Brack PDK for unsupported languages are welcome.

This is the main source code repository for Brack.
It contains the compiler, language server, project manager and document.
The following are the related repositories for Brack.

- [brack-lang/brack-pdk-rs](https://github.com/brack-lang/brack-pdk-rs)
    - PDK (Plug-in Development Kit) for Rust
- [brack-lang/vscode-brack](https://github.com/brack-lang/vscode-brack)
    - VSCode extension for Brack

## Syntax
Brack has 3 types of syntax which are an inline command, a block command and a macro.
They are called "Command Caller Syntax".

```brack
{sample.heading lipsum}

[sample.bold Lorem] ipsum dolor sit amet, consectetur
adipisicing elit, sed do eiusmod tempor
incididunt ut labore et dolore magna
aliqua.<sample.footnote for work and great pain>

{sample.hr}

[sample.italic Ut] enim ad minim veniam,
quis nostrud exercitation ullamco laboris
nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in 
[sample.anchor reprehenderit, https://en.wiktionary.org/wiki/reprehenderit]
in voluptate velit esse cillum dolore
eu fugiat nulla pariatur. Excepteur sint
occaecat cupidatat non proident,
sunt in culpa qui officia deserunt mollit anim id est laborum.
```

It converts from name and more than zero arguments when the command caller syntax is called.
It is identified by module name identifies plugins and command name identifies commands.

```brack
{module.inline-command arg1, arg2, ..., argN}
[module.block-command]
<module.macro arg1, arg2>
```

The Brack compiler does not implement specific conversions.
Therefore the compiler will raise an error if it doesn't load any plugins.
To convert a document, you can use the `std` plugin corresponding to the target format provided by the Brack development team, use third party plugins, or develop your own plugin.

For example, the following document is converted using [std.html](https://github.com/brack-lang/std.html), which is a plugin targeting HTML.

```brack
{std.* Hello, World!}

Hello, this is the document written using [std.* Brack]
<std.^ A markup language whose transformation rules can be extended by WebAssembly>.
```

std.html converts the above document to the following[^not-guarantee].

[^not-guarantee]: The conversion result depends on the version of the std.html plugin or the brack compiler. This is only an example and we do not guarantee that it will be converted in this way. You can get the information from the plugin document and the language server.

```html
<h1>Hello, World!</h1>
<p>
    Hello, this is the document written using <b>Brack</b>
    <sup>
        <a href="#fn-12345">[1]</a>
    </sup>.
</p>
<div>
    <div class="footnote-header">Footnotes</div>
    <ol class="footnote_ordered_list">
        <li id="fn-12345">
            A markup language whose transformation rules can be extended by WebAssembly
        </li>
    </ol>
</div>
```

## Features

- Minimal syntax and low learning curve
    - Even if you’ve used other markup languages before, it’s easy to start using Brack.
    - You can see how your document will be transformed through a language server, so there’s no need to constantly switch between a cheat sheet and your editor.
- Plugin implementation via WebAssembly
    - In Markdown, bold text (`**text**`) is converted to an HTML `b` tag (`<b>text</b>`). Many existing markup languages are not designed with the idea of extending such syntax rules.
    - In many languages that do support syntax extensions, plugins need to be written in the same language as the interpreter, but Brack overcomes this limitation by calling WebAssembly from the compiler.
    - Additionally, in Brack, **commands**—which are uniquely defined by name and type signature, rather than syntax—can be extended. This prevents an increase in learning cost or limitations on usable characters, which is crucial in a lightweight markup language.
- No restrictions on target formats
    - Markdown, depending on the interpreter, is mainly used to convert documents to HTML, but Brack imposes no such restrictions on target formats.
    - As of version 0.2.0, it can convert to any format as long as it's text-based, including HTML, LaTeX, and Pandoc Filters. You can also define special commands for containers like expressions or statements[^container-hook].
    - Support is also planned for post-processing mechanisms to output binary formats such as PDFs or videos.
- Project management tools and language server provided
    - The `brack` command includes both project management tools and a language server.
    - There’s no need to manage them separately; once installed, you can start using them immediately.

[^container-hook]: We call it Container Hooks.

## Installation

### Installation via Cargo (Rust’s package manager)
```sh
cargo install --git https://github.com/brack-lang/brack brack
```

### Installation via Nix
```sh
nix profile install github:brack-lang/brack
```

## Build

### Building with Nix (Recommended)
You can also build with Nix by enabling [Flakes](https://wiki.nixos.org/wiki/Flakes).
GitHub Actions automatically build and test using Nix, so building with Nix is recommended.

```sh
git clone https://github.com/brack-lang/brack.git
cd brack
nix build .

# Alternatively
echo "nix flake" > .envrc
direnv allow
```

### Building with Cargo

```sh
git clone https://github.com/brack-lang/brack.git
cd brack
cargo build --release
```

## LICENSE
All files within the [assets](./assets) and [doc](./doc) directories are licensed under CC-BY-4.0.
The remaining files are licensed under the Apache License 2.0 or the MIT License.
