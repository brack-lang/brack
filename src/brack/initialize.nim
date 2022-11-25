import std/macros
import std/macrocache
import api
import ast except newIdentNode

func analyzeArguments* (formalParams: NimNode): seq[string] {.compileTime.} =
  var params = formalParams.copy
  params.del(0)
  for param in params:
    let types = if param[^2].kind == nnkIdent and param[^2].strVal == "string": "string"
                elif param[^2].kind == nnkBracketExpr and param[^2][0].strVal == "seq" and param[^2][1].strVal == "string": "seq[string]"
                else: raise newException(Defect, "非対応の型")
    var param = param.copy
    param.del(param.len - 1)
    param.del(param.len - 1)
    for i in 0 ..< param.len:
      result.add types

func getCommandBranch*: NimNode =
  result = nnkIfStmt.newTree()
  for command in mcCommandSyms:
    var callAST = nnkCall.newTree(
      command[0][1]
    )
    var defineRemainingArguments = newStmtList()
    for index, types in analyzeArguments(command[3]):
      if types == "string":
        callAST.add nnkBracketExpr.newTree(
          newIdentNode("arguments"),
          newLit(index)
        )
      elif types == "seq[string]":
        defineRemainingArguments = nnkVarSection.newTree(
          nnkIdentDefs.newTree(
            newIdentNode("remainingArguments"),
            newEmptyNode(),
            nnkBracketExpr.newTree(
              newIdentNode("arguments"),
              nnkInfix.newTree(
                newIdentNode("..^"),
                newLit(index),
                newLit(1)
              )
            )
          )
        )
        callAST.add newIdentNode("remainingArguments")
    result.add nnkElifBranch.newTree(
      nnkInfix.newTree(
        newIdentNode("=="),
        newIdentNode("procedureName"),
        newLit($command[0][1])
      ),
      nnkStmtList.newTree(
        defineRemainingArguments,
        nnkInfix.newTree(
          newIdentNode("&="),
          newIdentNode("result"),
          callAST
        )
      )
    )

func getMacroBranch*: NimNode =
  result = nnkIfStmt.newTree()
  if mcMacroSyms.len == 0:
    return newStmtList()
  for m in mcMacroSyms:
    var callAST = nnkCall.newTree(
      m[0][1]
    )
    callAST.add newIdentNode("result")
    callAST.add newIdentNode("id")
    result.add nnkElifBranch.newTree(
      nnkInfix.newTree(
        newIdentNode("=="),
        newIdentNode("procedureName"),
        newLit($m[0][1])
      ),
      nnkStmtList.newTree(
        nnkInfix.newTree(
          newIdentNode("="),
          newIdentNode("result"),
          callAST
        )
      )
    )

macro initGenerator* (name: static[string], T: typedesc, body: untyped): untyped =
  let
    commandBranchAST = getCommandBranch()
  result = quote do:
    proc commandGenerator (ast: BrackNode, prefix: string): T =
      var
        procedureName {.inject.} = ""
        arguments {.inject.} : seq[string] = @[]
      for node in ast.children:
        if node.kind == bnkIdent:
          procedureName = prefix & resolveProcedureName(node.val)
        elif node.kind == bnkArgument:
          var argument = ""
          for argNode in node.children:
            if argNode.kind == bnkCurlyBracket:
              argument.add commandGenerator(argNode, &"curly_{`name`}_")
            elif argNode.kind == bnkSquareBracket:
              argument.add commandGenerator(argNode, &"square_{`name`}_")
            elif argNode.kind == bnkText:
              argument.add argNode.val
          arguments.add argument
      `commandBranchAST`
    
    proc generate* (ast: BrackNode): T {.inject.} =
      for node in ast.children:
        result &= commandGenerator(node, &"curly_{`name`}_")

macro initBrack* (): untyped =
  let
    commandBranchAST = getCommandBranch()
    macroBranchAST = getMacroBranch()

  result = quote do:
    proc otherwiseMacroExpander (ast, node: BrackNode, id: string): BrackNode
    proc angleBracketMacroExpander (ast, node: BrackNode, id: string): BrackNode =
      # TODO: ここでastがそのまま使われていて更新結果が反映されていない
      result = ast
      var
        procedureName {.inject.} = ""
        id {.inject.} = id
      for childNode in node.children:
        if childNode.kind == bnkIdent:
          procedureName = "angle_" & resolveProcedureName(childNode.val)
        elif childNode.kind == bnkArgument:
          for argNode in childNode.children:
            case argNode.kind
            of bnkAngleBracket:
              result = angleBracketMacroExpander(result, argNode, argNode.id)
            of bnkSquareBracket, bnkCurlyBracket:
              result = otherwiseMacroExpander(result, argNode, argNode.id)
            else: discard
      `macroBranchAST`
    
    proc otherwiseMacroExpander (ast, node: BrackNode, id: string): BrackNode =
      result = ast
      for childNode in node.children:
        if childNode.kind == bnkAngleBracket:
          result = angleBracketMacroExpander(result, childNode, childNode.id)
        elif childNode.kind == bnkArgument:
          for argNode in childNode.children:
            case argNode.kind
            of bnkAngleBracket:
              result = angleBracketMacroExpander(result, argNode, argNode.id)
            of bnkSquareBracket, bnkCurlyBracket:
              result = otherwiseMacroExpander(result, argNode, argNode.id)
            else: discard

    proc expand* (node: BrackNode): BrackNode {.inject.} =
      # TODO: マクロが0になるまで展開を繰り返す
      result = node
      for childNode in node.children:
        if childNode.kind == bnkAngleBracket:
          result = angleBracketMacroExpander(result, childNode, childNode.id)
        else:
          result = otherwiseMacroExpander(result, childNode, childNode.id)

    proc commandGenerator (ast: BrackNode, prefix: string): string =
      var
        procedureName {.inject.} = ""
        arguments {.inject.} : seq[string] = @[]
      for node in ast.children:
        if node.kind == bnkIdent:
          procedureName = prefix & resolveProcedureName(node.val)
        elif node.kind == bnkArgument:
          var argument = ""
          for argNode in node.children:
            if argNode.kind == bnkCurlyBracket:
              argument.add commandGenerator(argNode, "curly_")
            elif argNode.kind == bnkSquareBracket:
              argument.add commandGenerator(argNode, "square_")
            elif argNode.kind == bnkText:
              argument.add argNode.val
          arguments.add argument
      `commandBranchAST`
    
    proc generate* (ast: BrackNode): string {.inject.} =
      for node in ast.children:
        result &= commandGenerator(node, "curly_")
