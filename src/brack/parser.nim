import ast
import types/tokenizer
import fusion/matching
import std/options

{.experimental: "strictFuncs".}

type
    ParsedContext = (BrackNode, seq[Token])
    TokenState = tuple[head: Token, tail: seq[Token]]

proc parseExprSeq (tokens: seq[Token]): Option[ParsedContext]
proc parseExpr (tokens: seq[Token]): Option[ParsedContext]
proc parseAngle (tokens: seq[Token]): Option[ParsedContext]
proc parseCurly (tokens: seq[Token]): Option[ParsedContext]
proc parseSquared (tokens: seq[Token]): Option[ParsedContext]
proc parseCommandSpec (tokens: seq[Token]): Option[ParsedContext]
proc parseArguments (tokens: seq[Token]): Option[ParsedContext]

func `@` (tokens: seq[Token]): TokenState =
    if tokens.len == 0:
        return (Token(kind: tkEmpty), @[])
    if tokens.len == 1:
        return (tokens[0], @[])
    return (tokens[0], tokens[1..tokens.len-1])

func pop (tokens: seq[Token]): Option[TokenState] =
    if tokens.len == 0:
        return none[(Token, seq[Token])]()
    if tokens.len == 1:
        return some((tokens[0], newSeq[Token]()))
    result = some((tokens[0], tokens[1..tokens.len-1]))

func check (tokens: seq[Token], value: string): bool =
    result = (tokens.len > 0) and (tokens[0].value == value)

func check (tokens: seq[Token], values: openArray[string]): bool =
  result = false
  for value in values:
    result = result or tokens.check(value)

func check (tokens: seq[Token], kind: TokenKind): bool =
    result = (tokens.len > 0) and (tokens[0].kind == kind)

func check (tokens: seq[Token], kinds: openArray[TokenKind]): bool =
  result = false
  for kind in kinds:
    result = result or tokens.check(kind)

func consume (tokens: seq[Token], value: string): (bool, seq[Token]) =
    let (head, tail) = @tokens
    if head.value == value:
        return (true, tail)
    return (false, tokens)

func consume (tokens: seq[Token], kind: TokenKind): (bool, seq[Token]) =
    let (head, tail) = @tokens
    if head.kind == kind:
        return (true, tail)
    return (false, tokens)

proc parseStmt (tokens: seq[Token]): Option[ParsedContext] =
    var
        newTokens = tokens
        resultAst = bnkStmt.newTree()

    if Some(@res) ?= parseCurly(newTokens):
        let (ast, newTokensFromCurly) = res
        newTokens = newTokensFromCurly
        resultAst.add(ast)
    elif Some(@res) ?= parseExprSeq(newTokens):
        let (ast, newTokensFromExprSeq) = res
        newTokens = newTokensFromExprSeq
        resultAst.add(ast)
    else:
        return none[ParsedContext]()

    while true:
        let (consumed, newTokensFromNewLine) = newTokens.consume(tkNewLine)
        if not consumed:
            break
        newTokens = newTokensFromNewLine

    result = some((resultAst, newTokens))

proc parseExprSeq (tokens: seq[Token]): Option[ParsedContext] =
    var
        newTokens = tokens
        ast = bnkExprSeq.newTree()
    ast.add(if Some(@res) ?= parseExpr(tokens):
                let (ast, newTokensFromExpr) = res
                newTokens = newTokensFromExpr
                ast
            else:
                return none[ParsedContext]())
    
    while newTokens.len > 0:
        if not newTokens.check("\n"):
            break

        let (_, newTokensFromNewline) = newTokens.consume("\n")
        newTokens = newTokensFromNewline

        let astFromExpr = if Some(@res) ?= parseExpr(newTokens):
                                let (ast, newTokensFromExpr) = res
                                newTokens = newTokensFromExpr
                                ast
                          else:
                            return none[ParsedContext]()
        ast.add(astFromExpr)

    return some((ast, newTokens))

proc parseExpr (tokens: seq[Token]): Option[ParsedContext] =
    var
        newTokens = tokens
        resultAst = bnkExpr.newTree()
    
    proc parseInner (tokens: seq[Token]): Option[ParsedContext] =
        if tokens.check(tkText) and (Some(@res) ?= tokens.pop):
            let (token, poppedNewTokens) = res
            some((newTextNode(token.value), poppedNewTokens))
        elif Some(@res) ?= parseAngle(tokens):
            some(res)
        elif Some(@res) ?= parseSquared(tokens):
            some(res)
        else:
            none[ParsedContext]()
    
    if Some(@res) ?= parseInner(newTokens):
        let (ast, newTokensFromInner) = res
        newTokens = newTokensFromInner
        resultAst.add(ast)
    else:
        return none[ParsedContext]()
    
    while newTokens.len > 0:
        if Some(@res) ?= parseInner(newTokens):
            let (ast, newTokensFromInner) = res
            newTokens = newTokensFromInner
            resultAst.add(ast)
        else:
            break
    
    result = some((resultAst, newTokens))

proc parseAngle (tokens: seq[Token]): Option[ParsedContext] =
    var (consumed, newTokens) = consume(tokens, tkAngleBracketOpen)
    if not consumed:
        return none[ParsedContext]()
    
    let call = if Some(@res) ?= parseCommandSpec(newTokens):
                    let (ast, newTokensFromCall) = res
                    newTokens = newTokensFromCall
                    ast
                else:
                    return none[ParsedContext]()
    
    (consumed, newTokens) = consume(newTokens, tkAngleBracketClose)
    if not consumed:
        return none[ParsedContext]()
    
    result = some((
        bnkAngleBracket.newTree(call),
        newTokens
    ))

proc parseCurly (tokens: seq[Token]): Option[ParsedContext] =
    var (consumed, newTokens) = consume(tokens, tkCurlyBracketOpen)
    if not consumed:
        return none[ParsedContext]()
    
    let call = if Some(@res) ?= parseCommandSpec(newTokens):
                    let (ast, newTokensFromCall) = res
                    newTokens = newTokensFromCall
                    ast
                else:
                    return none[ParsedContext]()
    
    (consumed, newTokens) = consume(newTokens, tkCurlyBracketClose)
    if not consumed:
        return none[ParsedContext]()
    
    result = some((
        bnkCurlyBracket.newTree(call),
        newTokens
    ))

proc parseSquared (tokens: seq[Token]): Option[ParsedContext] =
    var (consumed, newTokens) = consume(tokens, tkSquareBracketOpen)
    if not consumed:
        return none[ParsedContext]()

    let call = if Some(@res) ?= parseCommandSpec(newTokens):
                    let (ast, newTokensFromCall) = res
                    newTokens = newTokensFromCall
                    ast
                else:
                    return none[ParsedContext]()

    (consumed, newTokens) = consume(newTokens, tkSquareBracketClose)
    if not consumed:
        return none[ParsedContext]()

    result = some((
        bnkSquareBracket.newTree(call),
        newTokens
    ))

proc parseCommandSpec (tokens: seq[Token]): Option[ParsedContext] =
    var
        newTokens = tokens
        resultAst = bnkCommandSpec.newTree()

    proc parseInner (tokens: seq[Token]): Option[ParsedContext] =
        if tokens.check(tkText) and (Some(@res) ?= tokens.pop):
            let (token, poppedNewTokens) = res
            some((newTextNode(token.value), poppedNewTokens))
        elif Some(@res) ?= parseAngle(tokens):
            some(res)
        else:
            none[ParsedContext]()
    
    if Some(@res) ?= parseInner(newTokens):
        let (ast, newTokensFromInner) = res
        newTokens = newTokensFromInner
        resultAst.add(ast)
    else:
        return none[ParsedContext]()
    
    while newTokens.len > 0:
        if Some(@res) ?= parseInner(newTokens):
            let (ast, newTokensFromInner) = res
            newTokens = newTokensFromInner
            resultAst.add(ast)
        else:
            break

    if newTokens.check(" "):
        let (consumed, newTokenFromWhiteSpace) = newTokens.consume(" ")
        if not consumed:
            return none[ParsedContext]()
        newTokens = newTokenFromWhiteSpace

        resultAst.add(if Some(@res) ?= parseArguments(tokens):
                            let (ast, newTokensFromArguments) = res
                            newTokens = newTokensFromArguments
                            ast
                        else:
                            return none[ParsedContext]())
    
    result = some((resultAst, newTokens))

proc parseArguments (tokens: seq[Token]): Option[ParsedContext] =
    var
        newTokens = tokens
        resultAst = bnkArguments.newTree()
    resultAst.add(if Some(@res) ?= parseExpr(tokens):
                        let (ast, newTokensFromExpr) = res
                        newTokens = newTokensFromExpr
                        ast
                    else:
                        return none[ParsedContext]())
    
    while newTokens.len > 0:
        if not newTokens.check(","):
            break

        let (consumed, newTokensFromComma) = newTokens.consume(",")
        if not consumed:
            return none[ParsedContext]()
        newTokens = newTokensFromComma

        if Some(@res) ?= parseExpr(newTokens):
            let (ast, newTokensFromExpr) = res
            newTokens = newTokensFromExpr
            resultAst.add(ast)
        else:
            return none[ParsedContext]()
    
    result = some((resultAst, newTokens))

proc parse* (tokens: seq[Token]): Option[BrackNode] =
    var
        newTokens = tokens
        resultAst = bnkDocument.newTree()
    
    while newTokens.len > 0:
        if Some(@res) ?= parseStmt(newTokens):
            let (ast, newTokensFromStmt) = res
            newTokens = newTokensFromStmt
            resultAst.add(ast)
        else:
            return none[BrackNode]()
    
    result = some(resultAst)
