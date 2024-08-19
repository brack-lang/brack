# Brack

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
$ cargo install https://github.com/brack-lang/brack
```

### Nix
```sh
$ nix develop github:brack-lang/brack
```

## LICENSE
MIT OR Apache-2.0

