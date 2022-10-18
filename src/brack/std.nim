from std/htmlgen import nil
import std/oids
import std/strformat
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
  
  proc id* (text, id: string): string {.square: "&".} =
    result = htmlgen.span(text, id=id)

  proc footnoteSup* (text: string): string {.square: "footnoteSup".} =
    result = htmlgen.sup(text)

  proc footnoteFooter* (text: string): string {.square: "footnoteFooter".} =
    result = htmlgen.div(
      htmlgen.p(text)
    )

  proc footnote* (ast: BrackNode, id: string): BrackNode {.angle: "^".} =
    let
      text = ast.find(id).arguments[0].val
      n = ast.count(bnkSquareBracket, "footnoteSup")
      sup = bnkSquareBracket.newTree(
        newIdentNode("footnoteSup"),
        bnkArgument.newTree(
          bnkSquareBracket.newTree(
            newIdentNode("@"),
            bnkArgument.newTree(
              newTextNode(&"[{$n}]")
            ),
            bnkArgument.newTree(
              newTextNode(&"#{$n}")
            )
          )
        ),
      )
  
    result = ast.insert(id, sup).delete(id)
    if not ast.exists("footnote"):
      result.children.add BrackNode(
        id: "footnote",
        kind: bnkParagraph,
        children: @[
          newTextNode("脚注"),
          bnkSquareBracket.newTree(
            newIdentNode("footnoteFooter"),
            bnkArgument.newTree(
              newTextNode("")
            )
          )
        ]
      )
    result["footnote"] = bnkParagraph.newTree(
      newTextNode("脚注"),
      bnkSquareBracket.newTree(
        newIdentNode("footnoteFooter"),
        bnkArgument.newTree(
          newTextNode(result.find("footnote").children[1].children[1].children[0].val & "\n[" & $n & "]: " & text)
        )
      )
    )

  proc image* (url, alt: string): string {.curly: "img".} =
    result = htmlgen.img(src=url, alt=alt)
