import std/macros
import std/macrocache
import api
import parser

func getNumberOfArguments* (formalParams: NimNode): int {.compileTime.} =
  result = formalParams.len - 1
  for param in formalParams:
    if param.kind != nnkIdentDefs: continue
    result += param.len - 3

func getCommandBranch* (): NimNode =
  result = nnkIfStmt.newTree()
  for command in mcCommandSyms:
    var callAST = nnkCall.newTree(
      command[4][0][1]
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

macro initBrack* (): untyped =
  for macroSym in mcMacroSyms:
    echo macroSym.astGenRepr

  let
    expander = newIdentNode("expander")
    generator = newIdentNode("generator")
    procedureName = newIdentNode("procedureName")
    arguments = newIdentNode("arguments")
    commandBranchAST = getCommandBranch()
  result = quote do:
    proc `expander`* (ast: BrackNode): BrackNode =
      # マクロが0になるまで展開を繰り返す
      discard

    proc commandGenerator (ast: BrackNode, prefix: string, root = true): string =
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
              argument.add commandGenerator(argNode, "square_", false)
            elif argNode.kind == bnkText:
              argument.add argNode.val
          `arguments`.add argument
      `commandBranchAST`
    
    proc squareBracketGenerator (ast: BrackNode, root = true): string =
      result = commandGenerator(ast, "square_", root)
    
    proc curlyBracketGenerator (ast: BrackNode, root = true): string =
      result = commandGenerator(ast, "curly_", root)

    proc paragraphGenerator (ast: BrackNode): string =
      for node in ast.children:
        if node.kind == bnkText:
          result &= node.val
        elif node.kind == bnkSquareBracket:
          result &= squareBracketGenerator(node)
    
    proc `generator`* (ast: BrackNode): BrackNode =
      for node in ast.children:
        if node.kind == bnkCurlyBracket:
          result &= curlyBracketGenerator(node)
        elif node.kind == bnkParagraph:
          result &= "<p>" & paragraphGenerator(node).replace("\n", "<br />") & "</p>"
    
