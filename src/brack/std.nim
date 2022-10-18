from std/htmlgen import nil
import std/oids
import api
import ast
import parser

brackModule:
  proc h1* (text: string): string {.curly: "*".} =
    result = htmlgen.h1(text)

  proc h2* (text: string): string {.curly: "**".} =
    result = htmlgen.h2(text)

  proc h3* (text: string): string {.curly: "***".} =
    result = htmlgen.h3(text)

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

  proc anchorLink* (text, url: string): string {.square: "@".} =
    result = htmlgen.a(text, href=url)

  proc strikeoutline* (text: string): string {.square: "~".} =
    const style = """
      text-decoration: line-through;
    """
    result = htmlgen.span(text, style=style)

  proc underline* (text: string): string {.square: "_".} =
    const style = """
      text-decoration: underline;
    """
    result = htmlgen.span(text, style=style)

  proc inlineCode* (text: string): string {.square: "#".} =
    const style = """
      display: inline-block;
      padding: 0.1em 0.25em;
      color: #444;
      background-color: #e7edf3;
      border-radius: 3px;
      border: solid 1px #d6dde4;
    """
    result = htmlgen.code(text, style=style)

  proc footnote* (ast: BrackNode, id: Oid): BrackNode {.angle: "^".} =
    var hint = bnkSquareBracket.newTree(
      newIdentNode("^"),
      bnkArgument.newTree(
        newTextNode(ast.find(id).arguments[0].val)
      )
    )
    result = ast.insert(id, hint).delete(id)
    echo ast.nth(id)

  proc image* (url, alt: string): string {.curly: "img".} =
    result = htmlgen.img(src=url, alt=alt)
