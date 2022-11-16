import std/strutils

proc tokenize* (path: string): seq[string] =
  let
    brackSrcFile = open(path)
    brackSrc = brackSrcFile.readAll
  brackSrcFile.close()

  var
    token = ""
    index = 0
    squareBracketNestCount = 0
    curlyBracketNestCount = 0
    angleBracketNestCount = 0
    searchingCommandName = false
    isEscaping = false

  while index < brackSrc.len:
    let targetChar = brackSrc[index]

    if isEscaping:
      isEscaping = false
      index += 1
      token.add targetChar
    elif targetChar == '\\':
      isEscaping = true
      index += 1
    elif targetChar == '<':
      angleBracketNestCount += 1
      if token != "":
        result.add token.strip
        token = ""
      result.add $targetChar
      index += 1
      searchingCommandName = true
    elif targetChar == '>' and angleBracketNestCount > 0:
      angleBracketNestCount -= 1
      if token != "":
        result.add token.strip
        token = ""
      result.add $targetChar
      index += 1
    elif targetChar == '[':
      squareBracketNestCount += 1
      if token != "":
        result.add token.strip
        token = ""
      result.add $targetChar
      index += 1
      searchingCommandName = true
    elif targetChar == ']' and squareBracketNestCount > 0:
      squareBracketNestCount -= 1
      if token != "":
        result.add token.strip
        token = ""
      result.add $targetChar
      index += 1
    elif targetChar == '{':
      curlyBracketNestCount += 1
      if token != "":
        result.add token.strip
        token = ""
      result.add $targetChar
      index += 1
      searchingCommandName = true
    elif targetChar == '}' and curlyBracketNestCount > 0:
      curlyBracketNestCount -= 1
      if token != "":
        result.add token
        token = ""
      result.add $targetChar
      index += 1
    elif targetChar == ',' and (squareBracketNestCount > 0 or curlyBracketNestCount > 0 or angleBracketNestCount > 0):
      result.add [token.strip, $targetChar]
      token = ""
      index += 1
    elif targetChar == ' ' and searchingCommandName:
      if token != "":
        result.add token.strip
        token = ""
        index += 1
      searchingCommandName = false
    elif targetChar == '\n':
      if token != "" and token.strip != "":
        result.add token.strip
        token = ""
      result.add "\n"
      index += 1
    else:
      token.add targetChar
      index += 1

  if token != "":
    result.add token.strip
