import std/macros
import std/macrocache
import api
import ast except newIdentNode

func analyzeArguments* (formalParams: NimNode): seq[string] {.compileTime.} =
  var params = formalParams.copy
  params.del(0)
  for param in params:
    let types = if param[^2].kind == nnkSym and param[^2].strVal == "string": "string"
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
      ident(command[0].strVal)
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
        newLit(command[0].strVal)
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
      ident(m[0].strVal)
    )
    callAST.add newIdentNode("result")
    callAST.add newIdentNode("id")
    result.add nnkElifBranch.newTree(
      nnkInfix.newTree(
        newIdentNode("=="),
        newIdentNode("procedureName"),
        newLit(m[0].strVal)
      ),
      nnkStmtList.newTree(
        nnkInfix.newTree(
          newIdentNode("="),
          newIdentNode("result"),
          callAST
        )
      )
    )

macro initExpander* (backend: static[BackendLanguage]): untyped =
  let
    macroBranchAST = getMacroBranch()
    backend = $backend
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
          procedureName = "angle_" & `backend` & "_" & resolveProcedureName(childNode.val)
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

import std/json

proc generateJson* (ast: BrackNode): JsonNode =
  case ast.kind
  of bnkText, bnkIdent:
    result = %* {
      "type": newJString($ast.kind),
      "val": newJString(ast.val)
    }
  else:
    result = %* { "type": newJString($ast.kind), "children": [] } 
    for child in ast.children:
      result["children"].add generateJson(child)

macro initGenerator* (backendKind: static[BackendLanguage]): untyped =
  let
    commandBranchAST = getCommandBranch()
    backend = $backendKind
  
  case backendKind
  of Html:
    result = quote do:
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
                argument.add commandGenerator(argNode, "curly_" & `backend` & "_")
              elif argNode.kind == bnkSquareBracket:
                argument.add commandGenerator(argNode, "square_" & `backend` & "_")
              elif argNode.kind == bnkText:
                argument.add argNode.val
            arguments.add argument
        `commandBranchAST`
      
      proc generate* (ast: BrackNode): string {.inject.} =
        for node in ast.children:
          result &= commandGenerator(node, "curly_" & `backend` & "_")
  of Json:
    result = quote do:
      proc generate* (ast: BrackNode): JsonNode {.inject.} =
        result = generateJson(ast)

macro initBrack* (backend: static[BackendLanguage]): untyped =
  result = quote do:
    initExpander(`backend`)
    initGenerator(`backend`)
