# Brack ([EN](./README.md) / JA)

![](./brack-header.png)

[![Release](https://img.shields.io/github/v/release/brack-lang/brack.svg)](https://github.com/user/repository)
[![CI](https://github.com/brack-lang/brack/actions/workflows/ci.yml/badge.svg)](https://github.com/brack-lang/brack/actions/workflows/ci.yml)

> [!CAUTION]
> Brackは実験的な言語でまだ始まったばかりのプロジェクトです。あなたの（インストール、使ってみること、問題を報告すること、Pull Requestを作成することなど、すべての）貢献をお待ちしております！

BrackはWebAssemblyを用いて拡張できる軽量マークアップ言語です。
括弧ベースのシンプルな文法を持ちます。

## 特徴
Brackは以下のような特徴を持っている、あるいは持つことを目標に開発されています。

- 3種類の括弧（`[], {}, <>`）をベースにした単純な文法
- WebAssemblyによって言語を拡張可能
- 複数のバックエンドフォーマットを選択可能
- ASTを操作できる強力なマクロシステム
- プラグインシステムやパッケージマネージャ、言語サーバを提供

## インストール
Brackはいくつかのパッケージマネージャを利用してインストールできます。

### Cargo
```sh
cargo install --git https://github.com/brack-lang/brack brack
```

### Nix
```sh
nix profile install github:brack-lang/brack
```

## ビルド
BrackをビルドするためにはFlakesを有効にしたNixが必要です。

```sh
nix build .
```

## 文法
3種類のコマンド呼び出し構文が定義されています。
モジュール名（1つのプラグインに対して1つ定められる）、コマンドの識別子名、引数を用いて、`[module.ident arg1, arg2, ..., argN]`のように記述します。
`[]`はインラインコマンド、`{}`はブロックコマンドに対応しており、文字列に置換されます。
また、`<>`はマクロに対応しており、コンパイル時に抽象構文木を異なる抽象構文木に変換することができます。

[std.html](https://github.com/brack-lang/std.html)プラグインを利用して、HTMLに変換されるBrack文章を以下に示します。

```brack
{std.* Hello, World!}
こんにちは、これは[std.* Brack]<std.^ WebAssemblyによって拡張可能なマークアップ言語>を使って記述された文章です。
```

Brackコンパイラ本体には、特定の変換を意味するコマンドは定義されていません。

## LICENSE
MIT OR Apache-2.0

