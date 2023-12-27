from std/options import some, none, Option, isNone, get, option

type
    TokenKind* = enum
        tkEmpty
        tkText
        tkNewLine
        tkAngleBracketOpen
        tkAngleBracketClose
        tkSquareBracketOpen
        tkSquareBracketClose
        tkCurlyBracketOpen
        tkCurlyBracketClose
        tkComma
    
    Token* = object
        kind*: TokenKind
        value*: string

    Context* = object
        untreated: Option[string] = none[string]()
        pool: Option[string] = none[string]()
        tokens: Option[seq[Token]] = none[seq[Token]]()
        escape: Option[bool] = none[bool]()
        angleNestCount: Option[int] = none[int]()
        squareNestCount: Option[int] = none[int]()
        curlyNestCount: Option[int] = none[int]()
        lookingForIdentifier: Option[bool] = none[bool]()

func `//`[T] (super, self: Option[T]): Option[T] =
    if self.isNone:
        return super
    return self

func `//`* (super, self: Context): Context =
    result = Context(
        untreated: super.untreated // self.untreated,
        pool: super.pool // self.pool,
        tokens: super.tokens // self.tokens,
        escape: super.escape // self.escape,
        angleNestCount: super.angleNestCount // self.angleNestCount,
        squareNestCount: super.squareNestCount // self.squareNestCount,
        curlyNestCount: super.curlyNestCount // self.curlyNestCount,
        lookingForIdentifier: super.lookingForIdentifier // self.lookingForIdentifier,
    )

func initContext* (untreated = none[string](),
                  pool = none[string](),
                  tokens = none[seq[Token]](),
                  escape = none[bool](),
                  angleNestCount = none[int](),
                  squareNestCount = none[int](),
                  curlyNestCount = none[int](),
                  lookingForIdentifier = none[bool]()): Context =
    result = Context(
        untreated: untreated,
        pool: pool,
        tokens: tokens,
        escape: escape,
        angleNestCount: angleNestCount,
        squareNestCount: squareNestCount,
        curlyNestCount: curlyNestCount,
        lookingForIdentifier: lookingForIdentifier,
    )

func untreated* (context: Context): string =
    result = context.untreated.get("")

func pool* (context: Context): string =
    result = context.pool.get("")

func tokens* (context: Context): seq[Token] =
    result = context.tokens.get(@[])

func escape* (context: Context): bool =
    result = context.escape.get(false)

func angleNestCount* (context: Context): int =
    result = context.angleNestCount.get(0)

func squareNestCount* (context: Context): int =
    result = context.squareNestCount.get(0)

func curlyNestCount* (context: Context): int =
    result = context.curlyNestCount.get(0)

func lookingForIdentifier* (context: Context): bool =
    result = context.lookingForIdentifier.get(false)
