from std/options import some, none, Option, isNone, get
from std/strutils import strip, isEmptyOrWhitespace
import types/tokenizer

{.experimental: "strictFuncs".}

func innerTokenize (context: Context): seq[Token]

func initToken* (kind: TokenKind, value: string): Token =
    result = Token(kind: kind, value: value)

func `@` (str: string): (char, string) =
    if str == "":
        return ('\0', "")
    if str.len == 1:
        return (str[0], "")
    return (str[0], str[1..str.len-1])

func updateTokens (context: Context, kind: TokenKind, strip: bool = false): seq[Token] =
    if context.pool == "":
        return context.tokens
    if strip:
        return context.tokens & initToken(kind, context.pool.strip)
    return context.tokens & initToken(kind, context.pool)

func tokenizeEscape (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = innerTokenize(context // initContext(
        untreated = some(tail),
        pool = some(context.pool & head),
        escape = some(false),
    ))

func tokenizeBackSlash (context: Context): seq[Token] =
    let (_, tail) = @(context.untreated)
    return innerTokenize(context // initContext(
        untreated = some(tail),
        escape = some(true),
    ))

func tokenizeAngleBracketOpen (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = innerTokenize(context // initContext(
        untreated = some(tail),
        pool = some(""),
        tokens = some(updateTokens(context, tkText) & initToken(tkAngleBracketOpen, $head)),
        angleNestCount = some(context.angleNestCount + 1),
        lookingForIdentifier = some(true),
    ))

func tokenizeAngleBracketClose (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = if context.lookingForIdentifier:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            lookingForIdentifier = some(false),
            tokens = some(updateTokens(context, tkText, true) & initToken(tkAngleBracketClose, $head)),
            angleNestCount = some(context.angleNestCount - 1),
        ))
    else:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            tokens = some(updateTokens(context, tkText, true) & initToken(tkAngleBracketClose, $head)),
            angleNestCount = some(context.angleNestCount - 1),
        ))

func tokenizeSquareBracketOpen (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = innerTokenize(context // initContext(
        untreated = some(tail),
        pool = some(""),
        tokens = some(updateTokens(context, tkText) & initToken(tkSquareBracketOpen, $head)),
        squareNestCount = some(context.squareNestCount + 1),
        lookingForIdentifier = some(true),
    ))

func tokenizeSquareBracketClose (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = if context.lookingForIdentifier:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            lookingForIdentifier = some(false),
            tokens = some(updateTokens(context, tkText, true) & initToken(tkSquareBracketClose, $head)),
            squareNestCount = some(context.squareNestCount - 1),
        ))
    else:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            tokens = some(updateTokens(context, tkText, true) & initToken(tkSquareBracketClose, $head)),
            squareNestCount = some(context.squareNestCount - 1),
        ))

func tokenizeCurlyBracketOpen (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = innerTokenize(context // initContext(
        untreated = some(tail),
        pool = some(""),
        tokens = some(updateTokens(context, tkText) & initToken(tkCurlyBracketOpen, $head)),
        curlyNestCount = some(context.curlyNestCount + 1),
        lookingForIdentifier = some(true),
    ))

func tokenizeCurlyBracketClose (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = if context.lookingForIdentifier:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            lookingForIdentifier = some(false),
            tokens = some(updateTokens(context, tkText, true) & initToken(tkCurlyBracketClose, $head)),
            curlyNestCount = some(context.curlyNestCount - 1),
        ))
    else:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            tokens = some(updateTokens(context, tkText, true) & initToken(tkCurlyBracketClose, $head)),
            curlyNestCount = some(context.curlyNestCount - 1),
        ))

func tokenizeArguments (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)
    result = innerTokenize(context // initContext(
        untreated = some(tail),
        pool = some(""),
        tokens = some(updateTokens(context, tkText, true) & initToken(tkComma, $head)),
    ))

func tokenizeIdentifier (context: Context): seq[Token] =
    let (_, tail) = @(context.untreated)
    result = innerTokenize(context // initContext(
        untreated = some(tail),
        pool = some(""),
        tokens = some(context.tokens & initToken(tkText, context.pool)),
        lookingForIdentifier = some(false),
    ))

func tokenizeNewLine (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)

    result = if context.pool.isEmptyOrWhitespace:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            tokens = some(context.tokens & initToken(tkNewLine, $head)),
        ))
    else:
        innerTokenize(context // initContext(
            untreated = some(tail),
            pool = some(""),
            tokens = some(context.tokens & initToken(tkText, context.pool) & initToken(tkNewLine, $head)),
        ))

func innerTokenize (context: Context): seq[Token] =
    let (head, tail) = @(context.untreated)

    if head == '\0':
        return updateTokens(context, tkText)
    
    if context.escape:
        return tokenizeEscape(context)
    
    if head == '\\':
        return tokenizeBackSlash(context)
    
    if head == '<':
        return tokenizeAngleBracketOpen(context)
    
    if head == '>' and context.angleNestCount > 0:
        return tokenizeAngleBracketClose(context)

    if head == '[':
        return tokenizeSquareBracketOpen(context)

    if head == ']' and context.squareNestCount > 0:
        return tokenizeSquareBracketClose(context)

    if head == '{':
        return tokenizeCurlyBracketOpen(context)

    if head == '}' and context.curlyNestCount > 0:
        return tokenizeCurlyBracketClose(context)

    if head == ',' and (context.squareNestCount + context.curlyNestCount + context.angleNestCount > 0):
        return tokenizeArguments(context)

    if head == ' ' and context.lookingForIdentifier:
        return tokenizeIdentifier(context)
    
    if head == '\n':
        return tokenizeNewLine(context)

    return innerTokenize(context // initContext(
        untreated = some(tail),
        pool = some(context.pool & head),
    ))

func tokenize* (text: string): seq[Token] =
    result = innerTokenize(initContext(
        untreated = some(text),
    ))
