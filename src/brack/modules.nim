import std/macros
import std/macrocache
import std/strformat

const
  mcCommandSyms = CacheSeq"CommandSyms"
  mcMacroSyms = CacheSeq"MacroSyms"

func getNumberOfArguments* (formalParams: NimNode): int {.compileTime.} =
  result = formalParams.len - 1
  for param in formalParams:
    if param.kind != nnkIdentDefs: continue
    result += param.len - 3

macro brackModule* (body: untyped): untyped =
  if body.kind == nnkProcDef:
    let
      name = $body[4][0][1]
      kind = $body[4][0][0]
    case kind
    of "square", "curly":
      mcCommandSyms.add newIdentNode(&"{kind}_{resolveProcedureName(name)}")
    of "angle":
      mcMacroSyms.add newIdentNode(&"{kind}_{resolveProcedureName(name)}")
