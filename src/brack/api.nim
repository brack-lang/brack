import std/macros
import std/macrocache

const
  mcCommandSyms* = CacheSeq"CommandSyms"
  mcMacroSyms* = CacheSeq"MacroSyms"

func resolveProcedureName* (command_name: string): string =
  for ch in command_name:
    result.add $int(ch)

macro brackModule* (stmtlist: untyped): untyped =
  for statement in stmtlist:
    if statement.kind == nnkProcDef:
      let kind = $statement[4][0][0]
      case kind
      of "square", "curly":
        mcCommandSyms.add statement
      of "angle":
        mcMacroSyms.add statement
  result = stmtlist

macro square* (name: static[string], body: untyped): untyped =
  result = copy(body)
  let procNameIdent = newIdentNode("square_" & resolveProcedureName(name))
  if result[0][1].kind == nnkAccQuoted:
    result[0][1][0] = procNameIdent
  elif result[0][1].kind == nnkIdent:
    result[0][1] = procNameIdent
  var privateProc = copy(body)
  privateProc[0] = privateProc[0][1]
  if privateProc[0].kind == nnkIdent:
    privateProc[0] = newIdentNode($privateProc[0])
  elif privateProc[0][0].kind == nnkAccQuoted:
    privateProc[0][0] = newIdentNode($privateProc[0][0])
  privateProc[4] = nnkPragma.newTree(
    newIdentNode("used")
  )
  result = newStmtList(result, privateProc)

macro curly* (name: static[string], body: untyped): untyped =
  result = copy(body)
  let procNameIdent = newIdentNode("curly_" & resolveProcedureName(name))
  if result[0][1].kind == nnkAccQuoted:
    result[0][1][0] = procNameIdent
  elif result[0][1].kind == nnkIdent:
    result[0][1] = procNameIdent

macro angle* (name: static[string], body: untyped): untyped =
  result = copy(body)
  let procNameIdent = newIdentNode("angle_" & resolveProcedureName(name))
  if result[0][1].kind == nnkAccQuoted:
    result[0][1][0] = procNameIdent
  elif result[0][1].kind == nnkIdent:
    result[0][1] = procNameIdent