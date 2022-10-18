# Brack

## 概要
Brackはカッコ（Brackets）を基本単位に持つ軽量マークアップ言語です。

Markdownは非常に有名でさまざまなユースケースで利用されます。
たとえば静的サイトジェネレータによって生成される個人ブログの文書はMarkdownで記述されることが多いでしょう。
軽量マークアップ言語はデータ記述の整合性を保ちつつ**可読性**が意識されていることが特徴ですが、ブログのような変換後のコンテンツのみが公開され重要である場合、ソース自体の可読性が強く意識される必要はありません。
弊害として、可読性が意識されることに重きを置かれ、記号が意味を代替していないことがあります（たとえば画像の`![]()`はリンクと本質的な区別が付かない上、`!`が画像の意味を持つことは非自明です）。
そればかりか、標準的なMarkdown規格（[CommonMark](https://commonmark.org/)など）では表現力が足りず実装が各々独自の拡張を加えることが多く、表現力が求められる場合においてMarkdownを使うことは本来の責務（たとえばこのREADMEやドキュメントなどソースコード自体も変換後のコンテンツも読まれるような目的に対する記述）から逸脱しています。

BrackはMarkdownをはじめとする軽量マークアップ言語の記述の容易さ・可読性をある程度残しつつ、ユーザーによる機能拡張によって成り立つマークアップパーザです。

## インストール
Nim言語のパッケージマネージャーであるNimbleを利用してインストールします。

```sh
$ nimble install https://github.com/momeemt/brack
```

## 文法
### Paragraph（段落）
プレーンテキストはParagraphとして扱われます。改行（`\n`）は素直に改行として変換されるため注意が必要です。

```brack
こんにちは。
Brackで文章を書いています。
```

対応するMarkdown

```md
こんにちは。

Brackで文章を書いています。
```

### CurlyBracket（波カッコ）
CurlyBracket（`{}`）はブロック構造です。

後述するSquareBracketやCurlyBracket内に入れ子にして展開することはできません。

```brack
{* 見出し1}
{** 見出し2}
{*** 見出し3}
```

対応するMarkdown

```md
# 見出し1
## 見出し2
### 見出し3
```

### SquareBracket（角カッコ）
SquareBracket（`[]`）はインライン構造であり、パラグラフ内に展開されます。

1行に対してこれが1つのみが存在する場合でも、空のパラグラフ内に文書が展開されます。
SquareBracketは、SquareBracketやCurlyBracket内に入れ子構造を取ることができます。

```brack
こんにちは。この文章は[* Brack]で書かれています。
[/ イタリック]な表現もできます。[* [/ 入れ子]]にしてみましょう。

{*** 見出し内で[/ 入れ子]にしてOK}
```

対応するMarkdown

```md
こんにちは。この文章は**Brack**で書かれています。
*イタリック*な表現もできます。***入れ子***にしてみましょう。

### 見出し内で*入れ子*にしてOK
```

### AngleBracket（マクロ）
AngleBracket（`<>`）はマクロです。
SquareBracket、CurlyBracketとは異なり、文書全体の抽象構文木（AST）とマクロのASTのIDを受け取り、文書全体のASTを返します。

コマンドは文字列を単に変換しますが、マクロは文書全体の抽象構文木を参照できるため文書の別の位置に文字を挿入したり既にある文書を変更できます。
たとえば、脚注を表現するために有効です。

```brack
こんにちは<^ この挨拶があなたに向けられているかどうかはわかりません>。
```

対応するMarkdown

```md
こんにちは[^1]。

[^1]: この挨拶があなたに向けられているかどうかはわかりません
```

## コマンド
先述の通り、ボールド、イタリック、見出しなどのコマンドはBrack本体には実装されていません。
標準ライブラリである`std`内で、これらのコマンドはNimプログラムによって実装されています。

```nim
proc h1* (text: string): string {.curly: "*".} =
  result = htmlgen.h1(text)

proc bold* (text: string): string {.square: "*".} =
  const style = """
    font-weight: bold;
  """
  result = htmlgen.span(text, style=style)

proc italic* (text: string): string {.square: "/".} =
  const style = """
    font-style: italic;
  """
  result = htmlgen.span(text, style=style)
```

### 2引数以上のコマンド
`square`、`curly`プラグマを付与したプロシージャはBrackでコマンドとして扱われます。
Brack内の識別子はプラグマに渡す文字列によって決定されるため、プロシージャ名は任意に決めることができます。
引数と戻り値の型は現状では`string`型である必要があります。

ここまでは`[cmd text]`のような1引数のコマンドのみを見てきましたが、2引数以上のコマンドを作成することもできます。
以下の`anchorLink`プロシージャは`@`というSquareBracketに対するコマンドで、2つの引数を受け取ります。
Brackはカンマ（`,`）で区切ることで複数の引数を渡すことができるため、`[@ momeemt, https://github.com/momeemt]`というコマンドを`<a href="https://github.com/momeemt">momeemt</a>`に変換できます。

```nim
proc anchorLink* (text, url: string): string {.square: "@".} =
  result = htmlgen.a(text, href=url)
```

### コマンドのエクスポート
作成したコマンドは提供されている`brackModule`マクロを使ってエクスポートします。

```nim
from htmlgen import nil
import brack

brackModule:
  proc red* (text: string): string {.square: "!!!".} =
    const style = """
      color: red;
    """
    result = htmlgen.span(text, style=style)
```

### ライブラリの読み込み

`lex`、`parse`、`expand`、`generate`という4つのプロシージャを順に組み合わせてBrackをHTMLに変換できます。

```nim
import brack
import brackStd
import brackMyLibrary

const Library = Base & MyLibrary
registerLibrary(Library)

var file = open("dist/hello_world.html", FileMode.fmWrite)
file.write(
  lex("hello_world.[]").parse().expand().generate()
)
outputFile.close()
```
