type
  BrackNodeKind* = enum
    bnkRoot
    bnkParagraph
    bnkSquareBracket
    bnkCurlyBracket
    bnkArgument
    bnkIdent
    bnkText
  
  BrackNode* = object
    case kind*: BrackNodeKind
    of bnkText, bnkIdent:
      val*: string
    else:
      children*: seq[BrackNode]

proc childrenToString (brackNode: seq[BrackNode], indentCount: int): string =
  var indent = ""
  for _ in 1 .. indentCount:
    indent &= "  "
  for child in brackNode:
    result &= indent & $child.kind & "\n"
    case child.kind
    of bnkText, bnkIdent:
      result &= indent & "  " & child.val & "\n"
    else:
      result &= childrenToString(child.children, indentCount+1)

proc `$`* (brackNode: BrackNode): string =
  result = $brackNode.kind & "\n"
  for child in brackNode.children:
    result &= "  " & $child.kind & "\n"
    case child.kind
    of bnkText, bnkIdent:
      result &= "    " & child.val & "\n"
    else:
      result &= childrenToString(child.children, 2)
  result = result[0..^2]

proc parseLeftSquareBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int] =
  var currentIndex = currentIndex
  while currentIndex < tokens.len:
    if tokens[currentIndex] == "{":
      raise newException(Defect, "squareBracket中にcurlyBracketが存在するのは許されない")
    elif tokens[currentIndex] == "[":
      let (children, newIndex) = parseLeftSquareBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add BrackNode(
          kind: bnkArgument
        )
      result.children[^1].children.add BrackNode(
        kind: bnkSquareBracket,
        children: children
      )
      currentIndex = newIndex
    elif tokens[currentIndex] == "]":
      result.index = currentIndex
      return
    elif tokens[currentIndex] == ",":
      result.children.add BrackNode(
        kind: bnkArgument
      )
    elif result.children.len == 0:
      result.children.add BrackNode(
        kind: bnkIdent,
        val: tokens[currentIndex]
      )
    else:
      if result.children.len == 1:
        result.children.add BrackNode(
          kind: bnkArgument
        )
      if tokens[currentIndex] == "\n":
        raise newException(Defect, "syntax error (squareBracket中に改行は許されない)")
      result.children[^1].children.add BrackNode(
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
          kind: bnkArgument
        )
      result.children[^1].children.add BrackNode(
        kind: bnkSquareBracket,
        children: children
      )
      currentIndex = newIndex
    elif tokens[currentIndex] == "}":
      result.index = currentIndex
      return
    elif tokens[currentIndex] == ",":
      result.children.add BrackNode(
        kind: bnkArgument
      )
    elif result.children.len == 0:
      result.children.add BrackNode(
        kind: bnkIdent,
        val: tokens[currentIndex]
      )
    else:
      if result.children.len == 1 and tokens[currentIndex] != "\n":
        result.children.add BrackNode(
          kind: bnkArgument
        )
      if tokens[currentIndex] != "\n" and tokens[currentIndex] != "":
        result.children[^1].children.add BrackNode(
          kind: bnkText,
          val: tokens[currentIndex]
        )
    currentIndex += 1

proc parse* (tokens: seq[string]): BrackNode =
  result = BrackNode(kind: bnkRoot)
  var index = 0
  while index < tokens.len:
    if tokens[index] == "[":
      var node = BrackNode(kind: bnkSquareBracket)
      (node.children, index) = parseLeftSquareBracket(tokens, index+1)
      result.children[^1].children.add node
    elif tokens[index] == "{":
      var node = BrackNode(kind: bnkCurlyBracket)
      (node.children, index) = parseLeftCurlyBracket(tokens, index+1)
      result.children.add node
    elif tokens[index] == "\n":
      var node = BrackNode(kind: bnkParagraph)
      result.children.add node
    else: 
      var node = BrackNode(kind: bnkText, val: tokens[index])
      result.children[^1].children.add node

    index += 1
