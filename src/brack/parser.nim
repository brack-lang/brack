import ast
  
proc parseLeftSquareBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int]
proc parseLeftCurlyBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int]

proc parseLeftAngleBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int] =
  var currentIndex = currentIndex
  while currentIndex < tokens.len:
    let token = tokens[currentIndex]
    if token == "<":
      let (children, newIndex) = parseLeftAngleBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add bnkArgument.newTree()
      result.children[^1].children.add bnkAngleBracket.newTree(children)
      currentIndex = newIndex
    elif token == "{":
      let (children, newIndex) = parseLeftCurlyBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add bnkArgument.newTree()
      result.children[^1].children.add bnkCurlyBracket.newTree(children)
      currentIndex = newIndex
    elif token == "[":
      let (children, newIndex) = parseLeftSquareBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add bnkArgument.newTree()
      result.children[^1].children.add bnkSquareBracket.newTree(children)
      currentIndex = newIndex
    elif token == ">":
      result.index = currentIndex
      return
    elif token == ",":
      result.children.add bnkArgument.newTree()
    elif result.children.len == 0:
      result.children.add newIdentNode(token)
    else:
      if result.children.len == 1 and token != "\n":
        result.children.add bnkArgument.newTree()
      if token != "\n" and token != "":
        result.children[^1].children.add newTextNode(token)
    currentIndex += 1

proc parseLeftSquareBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int] =
  var currentIndex = currentIndex
  while currentIndex < tokens.len:
    let token = tokens[currentIndex]
    if token == "{":
      raise newException(Defect, "squareBracket中にcurlyBracketが存在するのは許されない")
    elif token == "[":
      let (children, newIndex) = parseLeftSquareBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add bnkArgument.newTree()
      result.children[^1].children.add bnkSquareBracket.newTree(children)
      currentIndex = newIndex
    elif token == "]":
      result.index = currentIndex
      return
    elif token == ",":
      result.children.add bnkArgument.newTree()
    elif result.children.len == 0:
      result.children.add newIdentNode(token)
    else:
      if result.children.len == 1:
        result.children.add bnkArgument.newTree()
      if token == "\n":
        raise newException(Defect, "syntax error (squareBracket中に改行は許されない)")
      result.children[^1].children.add newTextNode(token)
    currentIndex += 1

proc parseLeftCurlyBracket (tokens: seq[string], currentIndex: int): tuple[children: seq[BrackNode], index: int] =
  var currentIndex = currentIndex
  while currentIndex < tokens.len:
    let token = tokens[currentIndex]
    if token == "{":
      let (children, newIndex) = parseLeftCurlyBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add bnkArgument.newTree()
      result.children[^1].children.add bnkCurlyBracket.newTree(children)
      currentIndex = newIndex
    elif token == "[":
      let (children, newIndex) = parseLeftSquareBracket(tokens, currentIndex + 1)
      if result.children.len == 1:
        result.children.add bnkArgument.newTree()
      result.children[^1].children.add bnkSquareBracket.newTree(children)
      currentIndex = newIndex
    elif token == "}":
      result.index = currentIndex
      return
    elif token == ",":
      result.children.add bnkArgument.newTree()
    elif result.children.len == 0:
      result.children.add newIdentNode(token)
    else:
      if result.children.len == 1 and token != "\n":
        result.children.add bnkArgument.newTree()
      if token != "\n" and token != "":
        result.children[^1].children.add newTextNode(token)
    currentIndex += 1

proc parse* (tokens: seq[string]): BrackNode =
  result = bnkRoot.newTree()
  var
    index = 0
    targetNode = newParagraph()
  while index < tokens.len:
    let token = tokens[index]
    if token == "<":
      var node = bnkAngleBracket.newTree()
      (node.children, index) = parseLeftAngleBracket(tokens, index+1)
      targetNode.children[^1].add node
    elif token == "[":
      var node = bnkSquareBracket.newTree()
      (node.children, index) = parseLeftSquareBracket(tokens, index+1)
      targetNode.children[^1].add node
    elif token == "{":
      var node = bnkCurlyBracket.newTree()
      (node.children, index) = parseLeftCurlyBracket(tokens, index+1)
      result.children.add node
    elif index > 0 and tokens[index-1] == "\n" and token == "\n":
      if not targetNode.empty:
        var targetNode = targetNode
        if targetNode.children.len > 0 and targetNode.children[^1].children.len > 0:
          targetNode.children[^1].children = targetNode.children[^1].children[0..^2]
        result.children.add targetNode
      targetNode = newParagraph()
    elif not(targetNode.children[^1].children.len == 0 and token == "\n"):
      var node = newTextNode(token)
      targetNode.children[^1].add node

    index += 1
  if not targetNode.empty:
    result.children.add targetNode