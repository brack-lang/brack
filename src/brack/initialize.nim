import std/macros
import std/macrocache
import std/strutils
import std/oids
import api
import parser

func getNumberOfArguments* (formalParams: NimNode): int {.compileTime.} =
  result = formalParams.len - 1
  for param in formalParams:
    if param.kind != nnkIdentDefs: continue
    result += param.len - 3

func getCommandBranch*: NimNode =
  result = nnkIfStmt.newTree()
  for command in mcCommandSyms:
    var callAST = nnkCall.newTree(
      command[0][1]
    )
    for index in 0 ..< getNumberOfArguments(command[3]):
      callAST.add nnkBracketExpr.newTree(
        newIdentNode("arguments"),
        newLit(index)
      )
    result.add nnkElifBranch.newTree(
      nnkInfix.newTree(
        newIdentNode("=="),
        newIdentNode("procedureName"),
        newLit($command[0][1])
      ),
      nnkStmtList.newTree(
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
    callAST.add newIdentNode("ast")
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

macro initBrack* (): untyped =
  let
    expander = newIdentNode("expander")
    generate = newIdentNode("generate")
    procedureName = newIdentNode("procedureName")
    arguments = newIdentNode("arguments")
    ast = newIdentNode("ast")
    id = newIdentNode("id")
    commandBranchAST = getCommandBranch()
    macroBranchAST = getMacroBranch()
  result = quote do:
    proc otherwiseMacroExpander (ast, node: BrackNode, id: Oid): BrackNode
    
    proc angleBracketMacroExpander (ast, node: BrackNode, id: Oid): BrackNode =
      # TODO: ここでastがそのまま使われていて更新結果が反映されていない
      var
        `procedureName` = ""
        `ast` = ast
        `id` = id
      for childNode in node.children:
        if childNode.kind == bnkIdent:
          `procedureName` = "angle_" & resolveProcedureName(childNode.val)
        elif childNode.kind == bnkArgument:
          for argNode in childNode.children:
            case argNode.kind
            of bnkAngleBracket:
              var argNode = argNode
              argNode = angleBracketMacroExpander(ast, argNode, argNode.id)
            of bnkSquareBracket, bnkCurlyBracket:
              var argNode = argNode
              argNode = otherwiseMacroExpander(ast, argNode, argNode.id)
            else: discard
      `macroBranchAST`
    
    proc otherwiseMacroExpander (ast, node: BrackNode, id: Oid): BrackNode =
      # TODO: ここでastがそのまま使われていて更新結果が反映されていない
      for childNode in node.children:
        if childNode.kind == bnkAngleBracket:
          var childNode = childNode
          childNode = angleBracketMacroExpander(ast, childNode, childNode.id)
        elif childNode.kind == bnkArgument:
          for argNode in childNode.children:
            case argNode.kind
            of bnkAngleBracket:
              var argNode = argNode
              argNode = angleBracketMacroExpander(ast, argNode, argNode.id)
            of bnkSquareBracket, bnkCurlyBracket:
              var argNode = argNode
              argNode = otherwiseMacroExpander(ast, argNode, argNode.id) 
            else: discard
      result = ast

    proc `expander`* (node: BrackNode): BrackNode =
      # マクロが0になるまで展開を繰り返す
      result = node
      for childNode in node.children:
        if childNode.kind == bnkAngleBracket:
          result = angleBracketMacroExpander(result, childNode, childNode.id)
        else:
          result = otherwiseMacroExpander(result, childNode, childNode.id)

    proc commandGenerator (ast: BrackNode, prefix: string): string =
      var
        `procedureName` = ""
        `arguments`: seq[string] = @[]
      for node in ast.children:
        if node.kind == bnkIdent:
          `procedureName` = prefix & resolveProcedureName(node.val)
        elif node.kind == bnkArgument:
          var argument = ""
          for argNode in node.children:
            if argument != "":
              argument &= " & "
            if argNode.kind == bnkSquareBracket:
              argument.add commandGenerator(argNode, "square_")
            elif argNode.kind == bnkText:
              argument.add argNode.val
          `arguments`.add argument
      `commandBranchAST`
    
    proc squareBracketCommandGenerator (ast: BrackNode): string =
      result = commandGenerator(ast, "square_")
    
    proc curlyBracketCommandGenerator (ast: BrackNode): string =
      result = commandGenerator(ast, "curly_")

    proc paragraphCommandGenerator (ast: BrackNode): string =
      for node in ast.children:
        if node.kind == bnkText:
          result &= node.val
        elif node.kind == bnkSquareBracket:
          result &= squareBracketCommandGenerator(node)
    
    proc `generate`* (ast: BrackNode): string =
      for node in ast.children:
        if node.kind == bnkCurlyBracket:
          result &= curlyBracketCommandGenerator(node)
        elif node.kind == bnkParagraph:
          result &= "<p>" & paragraphCommandGenerator(node).replace("\n", "<br />") & "</p>"
