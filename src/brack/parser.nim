import std/strutils
import std/oids
import ast
  
proc parseLeftSquareBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int]
proc parseLeftCurlyBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int]

proc empty (node: BrackNode): bool =
  case node.kind
  of bnkText, bnkIdent:
    result = node.val == ""
  else:
    result = node.children.len == 0

proc childrenToString (brackNode: seq[BrackNode], indentCount: int): string =
  var indent = ""
  for _ in 1 .. indentCount:
    indent &= "  "
  for child in brackNode:
    result &= indent & $child.kind & " (" & $child.id & ")\n"
    case child.kind
    of bnkText, bnkIdent:
      result &= indent & "  " & child.val.replace("\n", " \\n ") & "\n"
    else:
      result &= childrenToString(child.children, indentCount+1)

proc `$`* (brackNode: BrackNode): string =
  result = $brackNode.kind & "\n"
  case brackNode.kind
  of bnkText, bnkIdent:
    result &= "    " & brackNode.val & "\n"
  else:
    result &= childrenToString(brackNode.children, 1)
  result = result[0..^2]

proc parseLeftAngleBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int] =
  var currentIndex = currentIndex
  while currentIndex < tokens.len:
    if tokens[currentIndex] == "<":
      let (children, newIndex) = parseLeftAngleBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      result.children[^1].children.add BrackNode(
        id: $genOid(),
        kind: bnkAngleBracket,
        children: children
      )
      currentIndex = newIndex
    elif tokens[currentIndex] == "{":
      let (children, newIndex) = parseLeftCurlyBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      result.children[^1].children.add BrackNode(
        id: $genOid(),
        kind: bnkCurlyBracket,
        children: children
      )
      currentIndex = newIndex
    elif tokens[currentIndex] == "[":
      let (children, newIndex) = parseLeftSquareBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      result.children[^1].children.add BrackNode(
        id: $genOid(),
        kind: bnkSquareBracket,
        children: children
      )
      currentIndex = newIndex
    elif tokens[currentIndex] == ">":
      result.index = currentIndex
      return
    elif tokens[currentIndex] == ",":
      result.children.add BrackNode(
        id: $genOid(),
        kind: bnkArgument
      )
    elif result.children.len == 0:
      result.children.add BrackNode(
        id: $genOid(),
        kind: bnkIdent,
        val: tokens[currentIndex]
      )
    else:
      if result.children.len == 1 and tokens[currentIndex] != "\n":
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      if tokens[currentIndex] != "\n" and tokens[currentIndex] != "":
        result.children[^1].children.add BrackNode(
          id: $genOid(),
          kind: bnkText,
          val: tokens[currentIndex]
        )
    currentIndex += 1

proc parseLeftSquareBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int] =
  var currentIndex = currentIndex
  while currentIndex < tokens.len:
    if tokens[currentIndex] == "{":
      raise newException(Defect, "squareBracket中にcurlyBracketが存在するのは許されない")
    elif tokens[currentIndex] == "[":
      let (children, newIndex) = parseLeftSquareBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      result.children[^1].children.add BrackNode(
        id: $genOid(),
        kind: bnkSquareBracket,
        children: children
      )
      currentIndex = newIndex
    elif tokens[currentIndex] == "]":
      result.index = currentIndex
      return
    elif tokens[currentIndex] == ",":
      result.children.add BrackNode(
        id: $genOid(),
        kind: bnkArgument
      )
    elif result.children.len == 0:
      result.children.add BrackNode(
        id: $genOid(),
        kind: bnkIdent,
        val: tokens[currentIndex]
      )
    else:
      if result.children.len == 1:
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      if tokens[currentIndex] == "\n":
        raise newException(Defect, "syntax error (squareBracket中に改行は許されない)")
      result.children[^1].children.add BrackNode(
        id: $genOid(),
        kind: bnkText,
        val: tokens[currentIndex]
      )
    currentIndex += 1

proc parseLeftCurlyBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int] =
  var currentIndex = currentIndex
  while currentIndex < tokens.len:
    if tokens[currentIndex] == "{":
      raise newException(Defect, "syntax error (curlyBracketの入れ子は許されない)")
    elif tokens[currentIndex] == "[":
      let (children, newIndex) = parseLeftSquareBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      result.children[^1].children.add BrackNode(
        id: $genOid(),
        kind: bnkSquareBracket,
        children: children
      )
      currentIndex = newIndex
    elif tokens[currentIndex] == "}":
      result.index = currentIndex
      return
    elif tokens[currentIndex] == ",":
      result.children.add BrackNode(
        id: $genOid(),
        kind: bnkArgument
      )
    elif result.children.len == 0:
      result.children.add BrackNode(
        id: $genOid(),
        kind: bnkIdent,
        val: tokens[currentIndex]
      )
    else:
      if result.children.len == 1 and tokens[currentIndex] != "\n":
        result.children.add BrackNode(
          id: $genOid(),
          kind: bnkArgument
        )
      if tokens[currentIndex] != "\n" and tokens[currentIndex] != "":
        result.children[^1].children.add BrackNode(
          id: $genOid(),
          kind: bnkText,
          val: tokens[currentIndex]
        )
    currentIndex += 1

proc parse* (tokens: seq[string]): BrackNode =
  result = BrackNode(id: $genOid(), kind: bnkRoot)
  var
    index = 0
    targetNode = BrackNode(id: $genOid(), kind: bnkParagraph)
  while index < tokens.len:
    if tokens[index] == "<":
      var node = BrackNode(id: $genOid(), kind: bnkAngleBracket)
      (node.children, index) = parseLeftAngleBracket(tokens, index+1)
      targetNode.children.add node
    elif tokens[index] == "[":
      var node = BrackNode(id: $genOid(), kind: bnkSquareBracket)
      (node.children, index) = parseLeftSquareBracket(tokens, index+1)
      targetNode.children.add node
    elif tokens[index] == "{":
      var node = BrackNode(id: $genOid(), kind: bnkCurlyBracket)
      (node.children, index) = parseLeftCurlyBracket(tokens, index+1)
      result.children.add node
    elif tokens[index] == "\n":
      if not targetNode.empty:
        result.children.add targetNode
      targetNode = BrackNode(id: $genOid(), kind: bnkParagraph)
    else: 
      var node = BrackNode(id: $genOid(), kind: bnkText, val: tokens[index])
      targetNode.children.add node

    index += 1
  if not targetNode.empty:
    result.children.add targetNode
