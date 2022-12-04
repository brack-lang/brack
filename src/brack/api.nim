import std/macros
import std/macrocache
import std/strutils
import std/strformat

const
  mcCommandSyms* = CacheSeq"CommandSyms"
  mcMacroSyms* = CacheSeq"MacroSyms"

type
  BackendLanguage* {.pure.} = enum
    Html = "html"
    Json = "json"

func resolveProcedureName* (commandName: string): string =
  for ch in command_name:
    result.add $int(ch)

macro brackModule* (backend: static[BackendLanguage], body: untyped): untyped =
  var stmtlist = body
  for statement in stmtlist:
    if statement.kind == nnkProcDef:
      let kind = $statement[4][0][0]
      var statement = statement
      statement[0][1] = newIdentNode(&"{kind}_{$backend}_" & resolveProcedureName($statement[4][0][1]))
      case kind
      of "square", "curly":
        mcCommandSyms.add statement
      of "angle":
        mcMacroSyms.add statement
  result = body

macro square* (name: static[string], body: untyped): untyped = body
macro curly* (name: static[string], body: untyped): untyped = body
macro angle* (name: static[string], body: untyped): untyped = body

func style* (str: string): string =
  var str = str
  str = str.strip
  if str[^1] == '\n':
    str.delete(result.high..result.high)
  for line in str.split('\n'):
    result.add line.strip
