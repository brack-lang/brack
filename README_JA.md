# 🐦‍⬛ The Brack Markup Language

![](./assets/brack-header.png)

<div align="center">

[English](./README.md) | **日本語**

[![Release](https://img.shields.io/github/v/release/brack-lang/brack.svg)](https://github.com/brack-lang/brack/tree/main)
[![Pre-Release](https://img.shields.io/github/v/release/brack-lang/brack.svg?include_prereleases&label=prerelease)](https://github.com/user/repository)
[![CI](https://github.com/brack-lang/brack/actions/workflows/ci.yml/badge.svg)](https://github.com/brack-lang/brack/actions/workflows/ci.yml)

[![Discord Invite Budge](https://dcbadge.limes.pink/api/server/cH94kqUMYH?style=flat)](https://discord.gg/cH94kqUMYH)
[![X Following Budge](https://img.shields.io/twitter/follow/:bracklanguage)](https://twitter.com/intent/user?screen_name=bracklanguage)

</div>

Brackは文法がとてもシンプルで、拡張性が非常に高い**マークアップ言語**です。
WebAssemblyバイナリを利用したプラグインシステムを備えていて、あなたが使い慣れたプログラミング言語[^1]を使って、自由にコマンドを追加できます。

[^1]: ただし、WebAssemblyにコンパイルでき、かつ[Extism](https://extism.org/)のPDK（Plug-in Development Kit）が実装されている必要があります。2024年10月16日現在、ExtismのPDKが提供されているプログラミング言語はRust、JavaScript、Go、Haskell、AssemblyScript、C、Zig、.NETの8種類です。また、v0.2.0時点では、BrackのPDKはRustのみを提供しています。未サポートの言語でExtism、BrackのPDKを実装する貢献は大歓迎です。

このリポジトリはBrackのコンパイラと言語サーバ、プロジェクト管理ツール、ドキュメントが含まれています。
以下にBrackに関連するリポジトリをリストします。

- [brack-lang/brack-pdk-rs](https://github.com/brack-lang/brack-pdk-rs)
    - Rust言語によるPDK（プラグイン開発キット）
- [brack-lang/vscode-brack](https://github.com/brack-lang/vscode-brack)
    - Brackによる執筆支援を行うVSCodeの拡張機能

## 文法
Brackには、インラインコマンドとブロックコマンド、マクロの3種類の文法があります。
これらをコマンド呼び出し構文と呼びます。

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

コマンド呼び出し構文が呼び出されると、名前と0個以上の引数を受け取って異なるデータに置換します。
プラグインを識別するための**モジュール名**と、コマンドを識別するための**コマンド名**によって識別されます。

```brack
{module.inline-command arg1, arg2, ..., argN}
[module.block-command]
<module.macro arg1, arg2>
```

Brackのコンパイラは特定の変換を実装していません。
つまり、プラグインを入れない状態ではすべてのコマンド呼び出しはエラーになります。
Brackの開発チームから提供される、各ターゲットに対応する`std`プラグインやサードパーティ製のプラグインを使ったり、自分自身でプラグインを開発することで文章を変換できます。

例として、HTMLターゲットのプラグインである[std.html](https://github.com/brack-lang/std.html)を利用して、変換される文章を以下に示します。

```brack
{std.* Hello, World!}

Hello, this is the document written using [std.* Brack]
<std.^ A markup language whose transformation rules can be extended by WebAssembly>.
```

std.htmlは、上の文章を次のように変換します[^not-guarantee]。

[^not-guarantee]: std.htmlプラグインやBrackコンパイラのバージョンによって変換結果は左右されます。あくまで一例であり、このように変換されることを保証するものではありません。具体的にどのように変換されるかは、プラグインのドキュメントか言語サーバから得られる情報を参照してください。

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

## 特徴

- 最小限の文法と小さな学習コスト
    - これまで他のマークアップ言語を使っていたとしても、Brackを使い始めることは簡単です。
    - 言語サーバによってどのように文章が変換されるのかを知ることができるので、チートシートとエディタを往復する必要ありません。
- WebAssemblyによってプラグインを実装できる
    - Markdownではbold（`**text**`）がHTMLにおける`b`タグ（`<b>text</b>`）に変換されます。既存のマークアップ言語の多くはこのような構文規則を拡張することを前提に言語が設計されていません。
    - 構文拡張をサポートする言語の多くは処理系と同じ言語でプラグインを書く必要がありますが、BrackではコンパイラがWebAssemblyを呼び出すことで実現するため、そのような制約がありません。
    - また、Brackは構文ではなく、名前と型シグネチャで一意に定まる**コマンド**のみを拡張できるため、学習コストを引き上げず、使える文字の制限も増えません。これは軽量マークアップ言語においてとても重要です。
- ターゲットフォーマットが制約されない
    - Markdownは処理系によりますが多くはHTMLに変換するために用いられますが、Brackは出力するターゲットフォーマットを制約しません。
    - v0.2.0時点では、HTMLやLaTeX、Pandoc Filterなど、テキストであればどのようなフォーマットにも変換できます。そのために式や文などのコンテナに対する特別なコマンド[^container-hook]も定義できます。
    - また、PDFや動画などのバイナリ形式を出力するための後処理機構のサポートも計画されています。
- プロジェクト管理ツール、言語サーバを提供
    - `brack`コマンドには、プロジェクト管理ツールや言語サーバが含まれています。
    - 個別に管理する必要がなく、一度インストールすればすぐに使い始めることができます。

[^container-hook]: コンテナフック（Container Hooks）と呼びます。

## インストール

### Cargo（Rust言語のパッケージマネージャ）によるインストール
```sh
cargo install --git https://github.com/brack-lang/brack brack
```

### Nixによるインストール
```sh
nix profile install github:brack-lang/brack
```

## ビルド

### Nixによるビルド(推奨)
[Flakes](https://wiki.nixos.org/wiki/Flakes)を有効にしたNixを使ってもビルドできます。
GitHub Actions上で、Nixによるビルドとテストが行われているため、こちらを利用したビルドを推奨しています。

```sh
git clone https://github.com/brack-lang/brack.git
cd brack
nix build .

# あるいは
echo "nix flake" > .envrc
direnv allow
```

### Cargoによるビルド

```sh
git clone https://github.com/brack-lang/brack.git
cd brack
cargo build --release
```

## LICENSE
[assets](./assets)と[doc](./doc)内のすべてのファイルは、CC-BY-4.0でライセンスされています。
残りのファイルは、Apache License 2.0 or MIT License でライセンスされています。
