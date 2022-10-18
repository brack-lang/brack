import std/oids
import parser

type
  BrackNodeFindError = object of ValueError

func findHelper (ast: BrackNode, id: string): (bool, BrackNode) =
  if ast.id == id:
    return (true, ast)
  if ast.kind == bnkText or ast.kind == bnkIdent:
    return (false, BrackNode())
  else:
    for node in ast.children:
      let (res, resNode) = findHelper(node, id)
      if res:
        return (res, resNode)

func find* (ast: BrackNode, id: string): BrackNode =
  let (res, resNode) = findHelper(ast, id)
  if res:
    return resNode
  else:
    raise newException(BrackNodeFindError, "faild to find ast, " & id)

func arguments* (ast: BrackNode): seq[BrackNode] =
  for child in ast.children:
    if child.kind == bnkArgument:
      return child.children

func nthHelper (ast: BrackNode, id: string, kind: BrackNodeKind, finished: bool): tuple[n: int, finished: bool] =
  if finished or ast.id == id:
    return (0, true)
  elif ast.kind == bnkText or ast.kind == bnkIdent:
    return (int(ast.kind == kind), false)
  else:
    var
      res = int(ast.kind == kind)
      resN = 0
      finished: bool
    for child in ast.children:
      (resN, finished) = nthHelper(child, id, kind, finished)
      res += resN
    return (res, finished)

func nth* (ast: BrackNode, id: string): int =
  # TODO: 展開済みマクロがあるのでうまく動作しない
  let kind = ast.find(id).kind
  let res = nthHelper(ast, id, kind, false)
  result = res.n

func count* (ast: BrackNode, kind: BrackNodeKind, name: string, parentKind = bnkInvalid): int =
  if ast.kind == bnkIdent and kind == parentKind and ast.val == name:
    return 1
  elif not (ast.kind == bnkIdent or ast.kind == bnkText):
    for child in ast.children:
      result += count(child, kind, name, ast.kind)
    if ast.kind == bnkRoot:
      result += 1

func walk (ast: BrackNode): BrackNode =
  if ast.kind == bnkText or ast.kind == bnkIdent:
    return ast
  else:
    var parent = BrackNode(kind: ast.kind, id: ast.id)
    for child in ast.children:
      parent.children.add walk(child)
    return parent

func deleteHelper (ast: BrackNode, id: string): BrackNode =
  if ast.kind == bnkText or ast.kind == bnkIdent:
    if ast.id != id:
      return ast
  else:
    var parent = BrackNode(kind: ast.kind, id: ast.id)
    for child in ast.children:
      let res = child.deleteHelper(id)
      if res.kind != bnkInvalid:
        parent.children.add res
    if parent.id != id:
      return parent

func delete* (ast: BrackNode, id: string): BrackNode =
  result = ast.deleteHelper(id)

func insertHelper* (ast: BrackNode, id: string, insertAst: BrackNode): BrackNode =
  if ast.kind == bnkText or ast.kind == bnkIdent:
    return ast
  else:
    var
      finded = false
      res = BrackNode(kind: ast.kind, id: ast.id)
    for child in ast.children:
      res.children.add child
      if child.id == id:
        finded = true
        res.children.add insertAst
    if finded:
      return res
    else:
      res = BrackNode(kind: ast.kind, id: ast.id)
      for child in ast.children:
        res.children.add insertHelper(child, id, insertAst)
      return res

func insert* (ast: BrackNode, id: string, insertAst: BrackNode): BrackNode =
  result = insertHelper(ast, id, insertAst)

proc newTree* (kind: BrackNodeKind, children: varargs[BrackNode]): BrackNode =
  result = BrackNode(
    id: $genOid(),
    kind: kind,
  )
  for child in children:
    result.children.add child

proc newIdentNode* (val: string): BrackNode =
  result = BrackNode(
    id: $genOid(),
    kind: bnkIdent,
    val: val
  )

proc newTextNode* (val: string): BrackNode =
  result = BrackNode(
    id: $genOid(),
    kind: bnkText,
    val: val
  )

func exists* (ast: BrackNode, id: string): bool =
  if ast.kind == bnkText or ast.kind == bnkIdent:
    return ast.id == id
  else:
    var res = ast.id == id
    for child in ast.children:
      res = res or exists(child, id)
    return res

proc assignHelper (ast: BrackNode, id: string, node: BrackNode): BrackNode =
  if ast.id == id:
    var node = node
    node.id = id
    return node
  elif ast.kind == bnkText or ast.kind == bnkIdent:
    return ast
  else:
    var parent = BrackNode(kind: ast.kind, id: ast.id)
    for child in ast.children:
      parent.children.add assignHelper(child, id, node)
    return parent

proc `[]=`* (ast: var BrackNode, id: string, node: BrackNode) =
  ast = assignHelper(ast, id, node)

import std/macros

macro quote* (body: untyped): untyped =
  echo body.astGenRepr
  result = nnkStmtList.newTree(
    nnkCall.newTree(
      macros.newIdentNode("BrackNode")
    )
  )