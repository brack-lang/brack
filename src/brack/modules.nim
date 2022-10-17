import std/macros
import std/macrocache
import std/strformat

const
  mcCommandSyms = CacheSeq"CommandSyms"
  mcMacroSyms = CacheSeq"MacroSyms"

func resolveProcedureName* (command_name: string): string =
  for ch in command_name:
    result.add $int(ch)

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
