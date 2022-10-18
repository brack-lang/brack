import std/oids
import parser

type
  BrackNodeFindError = object of ValueError

func findHelper (ast: BrackNode, id: Oid): (bool, BrackNode) =
  if ast.id == id:
    return (true, ast)
  if ast.kind == bnkText or ast.kind == bnkIdent:
    return (false, BrackNode())
  else:
    for node in ast.children:
      let (res, resNode) = findHelper(node, id)
      if res:
        return (res, resNode)

func find* (ast: BrackNode, id: Oid): BrackNode =
  let (res, resNode) = findHelper(ast, id)
  if res:
    return resNode
  else:
    raise newException(BrackNodeFindError, "faild to find ast")

func arguments* (ast: BrackNode): seq[BrackNode] =
  for child in ast.children:
    if child.kind == bnkArgument:
      return child.children

func nthHelper (ast: BrackNode, id: Oid, kind: BrackNodeKind, n: var int): bool =
  if ast.id == id:
    return true
  if ast.kind == bnkText or ast.kind == bnkIdent:
    if ast.kind == kind:
      n += 1
  else:
    if ast.kind == kind:
      n += 1
    for node in ast.children:
      let res = nthHelper(node, id, kind, n)
      if res:
        return res

func nth* (ast: BrackNode, id: Oid): int =
  let kind = ast.find(id).kind
  result = 0
  let _ = nthHelper(ast, id, kind, result)

func walk (ast: BrackNode): BrackNode =
  if ast.kind == bnkText or ast.kind == bnkIdent:
    return ast
  else:
    var parent = BrackNode(kind: ast.kind, id: ast.id)
    for child in ast.children:
      parent.children.add walk(child)
    return parent

func deleteHelper (ast: BrackNode, id: Oid): BrackNode =
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

func delete* (ast: BrackNode, id: Oid): BrackNode =
  result = ast.deleteHelper(id)

func insertHelper* (ast: BrackNode, id: Oid, insertAst: BrackNode): BrackNode =
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
      # debugEcho "--------------------"
      # debugEcho res
      # debugEcho "--------------------"
      return res

func insert* (ast: BrackNode, id: Oid, insertAst: BrackNode): BrackNode =
  result = insertHelper(ast, id, insertAst)

proc newTree* (kind: BrackNodeKind, children: varargs[BrackNode]): BrackNode =
  result = BrackNode(
    id: genOid(),
    kind: kind,
  )
  for child in children:
    result.children.add child

proc newIdentNode* (val: string): BrackNode =
  result = BrackNode(
    id: genOid(),
    kind: bnkIdent,
    val: val
  )

proc newTextNode* (val: string): BrackNode =
  result = BrackNode(
    id: genOid(),
    kind: bnkText,
    val: val
  )