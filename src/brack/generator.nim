import std/[macros, strformat]
import parser

func resolveProcedureName* (command_name: string): string =
  for ch in command_name:
    result.add $int(ch)

func getCommandBranch* (commands: seq[(string, int)]): NimNode =
  result = nnkIfStmt.newTree()
  for command in commands:
    var callAST = nnkCall.newTree(
      newIdentNode(command[0])
    )
    for index in 0 ..< command[1]:
      callAST.add nnkBracketExpr.newTree(
        newIdentNode("arguments"),
        newLit(index)
      )
    result.add nnkElifBranch.newTree(
      nnkInfix.newTree(
        newIdentNode("=="),
        newIdentNode("procedureName"),
        newLit(command[0])
      ),
      nnkStmtList.newTree(
        nnkInfix.newTree(
          newIdentNode("&="),
          newIdentNode("result"),
          callAST
        )
      )
    )

macro registerLibrary* (commands: static[seq[(string, int)]]): untyped =
  result = newStmtList()
  let
    generateIdent = newIdentNode("generate")
    procedureNameIdent = newIdentNode("procedureName")
    argumentsIdent = newIdentNode("arguments")
    commandBranchAST = getCommandBranch(commands)

  result.add quote do:
    proc bracketGenerator (ast: BrackNode, prefix: string, root = true): string =
      var
        `procedureNameIdent` = ""
        `argumentsIdent`: seq[string] = @[]
      for node in ast.children:
        if node.kind == bnkIdent:
          `procedureNameIdent` = prefix & resolveProcedureName(node.val)
        elif node.kind == bnkArgument:
          var argument = ""
          for argNode in node.children:
            if argument != "":
              argument &= " & "
            if argNode.kind == bnkSquareBracket:
              argument.add bracketGenerator(argNode, "squared_", false)
            elif argNode.kind == bnkText:
              argument.add argNode.val
          `argumentsIdent`.add argument
      `commandBranchAST`
    
    proc squaredBracketGenerator (ast: BrackNode, root = true): string =
      result = bracketGenerator(ast, "squared_", root)
    
    proc curlyBracketGenerator (ast: BrackNode, root = true): string =
      result = bracketGenerator(ast, "curly_", root)

    proc paragraphGenerator (ast: BrackNode): string =
      for node in ast.children:
        if node.kind == bnkText:
          result &= node.val
        elif node.kind == bnkSquareBracket:
          result &= squaredBracketGenerator(node)

    proc `generateIdent`* (ast: BrackNode): string =
      for node in ast.children:
        if node.kind == bnkCurlyBracket:
          result &= curlyBracketGenerator(node)
        elif node.kind == bnkParagraph:
          result &= "<p>" & paragraphGenerator(node) & "</p>"

macro squared* (name: static[string], body: untyped): untyped =
  result = copy(body)
  let procNameIdent = newIdentNode("squared_" & resolveProcedureName(name))
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

macro `curly`* (name: static[string], body: untyped): untyped =
  result = copy(body)
  let procNameIdent = newIdentNode("curly_" & resolveProcedureName(name))
  if result[0][1].kind == nnkAccQuoted:
    result[0][1][0] = procNameIdent
  elif result[0][1].kind == nnkIdent:
    result[0][1] = procNameIdent

func getNumberOfArguments* (formalParams: NimNode): int {.compileTime.} =
  result = formalParams.len - 1
  for param in formalParams:
    if param.kind != nnkIdentDefs: continue
    result += param.len - 3

macro exportBrackModule* (libname, body: untyped): untyped =
  var exportList: seq[(string, int)] = @[]
  for statement in body:
    if statement.kind == nnkProcDef:
      let
        name = $statement[4][0][1]
        kind = $statement[4][0][0]
        numberOfArguments = getNumberOfArguments(statement[3])
      if kind == "squared":
        exportList.add ("squared_" & resolveProcedureName(name), numberOfArguments)
      elif kind == "curly":
        exportList.add ("curly_" & resolveProcedureName(name), numberOfArguments)
  result = body
  var bracketAST = nnkBracket.newTree()
  for exportProc in exportList:
    bracketAST.add newLit(exportProc)
  result.add nnkConstSection.newTree(
    nnkConstDef.newTree(
      nnkPostfix.newTree(
        newIdentNode("*"),
        newIdentNode($libname)
      ),
      newEmptyNode(),
      nnkPrefix.newTree(
        newIdentNode("@"),
        bracketAST
      )
    )
  )
