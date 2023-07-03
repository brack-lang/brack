import std/strutils

type
  Tokenizer* = object
    rules: seq[TokenizeRule]
    nextChar: char
    escaped: bool
    squareBracketNestCount: int
    curlyBracketNestCount: int
    angleBracketNestCount: int
    searchingCommandName: bool
    token: string
    tokens: seq[string]

  TokenizeRule* = object
    match: proc (tokenizer: Tokenizer): bool
    apply: proc (tokenizer: var Tokenizer)

func addRule (tokenizer: Tokenizer, rule: TokenizeRule): Tokenizer =
  var tokenizer = tokenizer
  tokenizer.rules.add rule
  result = tokenizer

proc tokenize* (tokenizer: Tokenizer, document: string): seq[string] =
  var
    tokenizer = tokenizer
    index = 0
  while index < document.len:
    tokenizer.nextChar = document[index]
    for rule in tokenizer.rules:
      if rule.match(tokenizer):
        rule.apply(tokenizer)
        break
    index += 1
  if tokenizer.token != "":
    tokenizer.tokens.add tokenizer.token
  result = tokenizer.tokens

proc escapedRule: TokenizeRule =
  proc escapedMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.escaped

  proc escapedApplier (tokenizer: var Tokenizer) =
    tokenizer.token.add tokenizer.nextChar
    tokenizer.escaped = false
  
  result = TokenizeRule(match: escapedMatcher, apply: escapedApplier)

proc backslashRule: TokenizeRule =
  proc backslashMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == '\\'

  proc backslashApplier (tokenizer: var Tokenizer) =
    tokenizer.escaped = true

  result = TokenizeRule(match: backslashMatcher, apply: backslashApplier)

proc openAngleBracketRule: TokenizeRule =
  proc openAngleBracketMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == '<'

  proc openAngleBracketApplier (tokenizer: var Tokenizer) =
    tokenizer.angleBracketNestCount += 1
    if tokenizer.token != "":
      tokenizer.tokens.add tokenizer.token
      tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar
    tokenizer.searchingCommandName = true

  result = TokenizeRule(match: openAngleBracketMatcher, apply: openAngleBracketApplier)

proc closeAngleBracketRule: TokenizeRule =
  proc closeAngleBracketMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == '>' and tokenizer.angleBracketNestCount > 0

  proc closeAngleBracketApplier (tokenizer: var Tokenizer) =
    tokenizer.angleBracketNestCount -= 1
    if tokenizer.token != "":
      tokenizer.tokens.add tokenizer.token
      tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar

  result = TokenizeRule(match: closeAngleBracketMatcher, apply: closeAngleBracketApplier)

proc openSquareBracketRule: TokenizeRule =
  proc openSquareBracketMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == '['

  proc openSquareBracketApplier (tokenizer: var Tokenizer) =
    tokenizer.squareBracketNestCount += 1
    if tokenizer.token != "":
      tokenizer.tokens.add tokenizer.token
      tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar
    tokenizer.searchingCommandName = true

  result = TokenizeRule(match: openSquareBracketMatcher, apply: openSquareBracketApplier)

proc closeSquareBracketRule: TokenizeRule =
  proc closeSquareBracketMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == ']' and tokenizer.squareBracketNestCount > 0

  proc closeSquareBracketApplier (tokenizer: var Tokenizer) =
    tokenizer.squareBracketNestCount -= 1
    if tokenizer.token != "":
      tokenizer.tokens.add tokenizer.token.strip
      tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar

  result = TokenizeRule(match: closeSquareBracketMatcher, apply: closeSquareBracketApplier)

proc openCurlyBracketRule: TokenizeRule =
  proc openCurlyBracketMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == '{'

  proc openCurlyBracketApplier (tokenizer: var Tokenizer) =
    tokenizer.curlyBracketNestCount += 1
    if tokenizer.token != "":
      tokenizer.tokens.add tokenizer.token
      tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar
    tokenizer.searchingCommandName = true

  result = TokenizeRule(match: openCurlyBracketMatcher, apply: openCurlyBracketApplier)

proc closeCurlyBracketRule: TokenizeRule =
  proc closeCurlyBracketMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == '}' and tokenizer.curlyBracketNestCount > 0

  proc closeCurlyBracketApplier (tokenizer: var Tokenizer) =
    tokenizer.curlyBracketNestCount -= 1
    if tokenizer.token != "":
      tokenizer.tokens.add tokenizer.token
      tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar

  result = TokenizeRule(match: closeCurlyBracketMatcher, apply: closeCurlyBracketApplier)

proc argumentsRule: TokenizeRule =
  proc argumentsMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == ',' and (tokenizer.squareBracketNestCount > 0 or tokenizer.curlyBracketNestCount > 0 or tokenizer.angleBracketNestCount > 0)

  proc argumentsApplier (tokenizer: var Tokenizer) =
    tokenizer.tokens.add tokenizer.token.strip
    tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar

  result = TokenizeRule(match: argumentsMatcher, apply: argumentsApplier)

proc commandNameRule: TokenizeRule =
  proc commandNameMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == ' ' and tokenizer.searchingCommandName

  proc commandNameApplier (tokenizer: var Tokenizer) =
    if tokenizer.token != "":
      tokenizer.tokens.add tokenizer.token
      tokenizer.token = ""
    tokenizer.searchingCommandName = false

  result = TokenizeRule(match: commandNameMatcher, apply: commandNameApplier)

proc newLinesRule: TokenizeRule =
  proc newLinesMatcher (tokenizer: Tokenizer): bool =
    result = tokenizer.nextChar == '\n'

  proc newLinesApplier (tokenizer: var Tokenizer) =
    tokenizer.tokens.add tokenizer.token
    tokenizer.token = ""
    tokenizer.tokens.add $tokenizer.nextChar

  result = TokenizeRule(match: newLinesMatcher, apply: newLinesApplier)

proc otherwiseRule: TokenizeRule =
  proc otherwiseMatcher (tokenizer: Tokenizer): bool =
    result = true

  proc otherwiseApplier (tokenizer: var Tokenizer) =
    tokenizer.token.add tokenizer.nextChar

  result = TokenizeRule(match: otherwiseMatcher, apply: otherwiseApplier)

func newTokenizer* (): Tokenizer =
  result = Tokenizer()
    .addRule(escapedRule())
    .addRule(backslashRule())
    .addRule(openAngleBracketRule())
    .addRule(closeAngleBracketRule())
    .addRule(openSquareBracketRule())
    .addRule(closeSquareBracketRule())
    .addRule(openCurlyBracketRule())
    .addRule(closeCurlyBracketRule())
    .addRule(argumentsRule())
    .addRule(commandNameRule())
    .addRule(newLinesRule())
    .addRule(otherwiseRule())
