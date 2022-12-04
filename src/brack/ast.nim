import std/oids
import std/strformat
import std/strutils

{.experimental: "strictFuncs".}

type
  BrackError* = object of CatchableError
  BrackIndexError* = object of BrackError
  BrackNoChildrenError* = object of BrackError
  BrackNotFoundASTError* = object of BrackError

  BrackNodeKind* = enum
    bnkInvalid
    bnkRoot
    bnkSquareBracket
    bnkCurlyBracket
    bnkAngleBracket
    bnkArgument
    bnkIdent
    bnkText
  
  BrackNodeObj = object
    id: string
    case kind: BrackNodeKind
    of bnkText, bnkIdent:
      val: string
    else:
      children*: seq[BrackNode]
  
  BrackNode* = ref BrackNodeObj

func noChildrenErrorMsg (ast: BrackNode): string =
  result = &"The brack node kind of {ast.id} is {ast.kind}. It doesn't have children."

func indexErrorMsg (ast: BrackNode, index: int): string =
  result = &"index must be less than the child element of ast. index: {index} >= ast.children.len: {ast.children.len}"

func notFoundASTErrorMsg (ast: BrackNode, id: string): string =
  result = &"ast({ast.id}) doesn't have the ast has id({id})."

proc newTree* (kind: BrackNodeKind, id: string, children: varargs[BrackNode]): BrackNode =
  result = BrackNode(
    id: id,
    kind: kind,
  )
  for child in children:
    result.children.add child

proc newTree* (kind: BrackNodeKind, children: varargs[BrackNode]): BrackNode =
  result = kind.newTree($genOid(), children)

proc newIdentNode* (id: string, val: string): BrackNode =
  result = BrackNode(
    id: id,
    kind: bnkIdent,
    val: val
  )

proc newIdentNode* (val: string): BrackNode =
  result = newIdentNode($genOid(), val)

proc newParagraph* (id: string = $genOid()): BrackNode =
  result = bnkCurlyBracket.newTree()
  result.children.add newIdentNode(id, "paragraph")
  result.children.add bnkArgument.newTree()

proc newTextNode* (id: string, val: string): BrackNode =
  result = BrackNode(
    id: id,
    kind: bnkText,
    val: val
  )

proc newTextNode* (val: string): BrackNode =
  result = newTextNode($genOid(), val)

func hasChildren* (ast: BrackNode): bool =
  result = not (ast.kind == bnkText or ast.kind == bnkIdent)

func empty* (node: BrackNode): bool =
  case node.kind
  of bnkText, bnkIdent:
    result = node.val == ""
  else:
    result = node.children.len == 0

func `or` (n1, n2: BrackNode): BrackNode =
  if n1.isNil:
    result = n2
  else:
    result = n1

func find (ast: BrackNode, id: string): BrackNode =
  if ast.id == id:
    result = ast
  elif ast.hasChildren:
    var tmp: BrackNode = nil
    for child in ast.children:
      tmp = tmp or child.find(id)
    result = tmp
  else:
    result = nil

func `[]`* (ast: BrackNode, id: string): BrackNode =
  result = ast.find(id)
  if result.isNil:
    raise newException(BrackNotFoundASTError, notFoundASTErrorMsg(ast, id))

func `[]`* (ast: BrackNode, index: int): BrackNode =
  if ast.hasChildren:
    if ast[].children.len <=  index:
      raise newException(BrackIndexError, indexErrorMsg(ast, index))
    result = ast.children[index]
  else:
    raise newException(BrackNoChildrenError, noChildrenErrorMsg(ast))

proc set (ast: BrackNode, id: string, newAst: BrackNode) =
  if ast.id == id:
    ast[] = newAst[]
  elif ast.hasChildren:
    for child in ast.children:
      child.set(id, newAst)

proc `[]=`* (ast: BrackNode, id: string, newAst: BrackNode) =
  ast.set(id, newAst)
  if ast.isNil:
    raise newException(BrackNotFoundASTError, notFoundASTErrorMsg(ast, id))

proc `[]=`* (ast: BrackNode, index: int, newAst: BrackNode) =
  if ast.hasChildren:
    if ast[].children.len <= index:
      raise newException(BrackIndexError, indexErrorMsg(ast, index))
    ast[].children[index] = newAst
  else:
    raise newException(BrackNoChildrenError, noChildrenErrorMsg(ast))

func addIndent (s: string): string =
  for line in s.split('\n'):
    if line == "": continue
    result.add &"  {line}\n"

func `$`* (ast: BrackNode): string =
  result = &"{$ast.kind} ({$ast.id})\n"
  if ast.hasChildren:
    for child in ast.children:
      result &= addIndent($child)
  else:
    result.add ast.val.replace("\n", " \\n ").addIndent

# func nthHelper (ast: BrackNode, id: string, kind: BrackNodeKind, finished: bool): tuple[n: int, finished: bool] =
#   if finished or ast.id == id:
#     return (0, true)
#   elif ast.kind == bnkText or ast.kind == bnkIdent:
#     return (int(ast.kind == kind), false)
#   else:
#     var
#       res = int(ast.kind == kind)
#       resN = 0
#       finished: bool
#     for child in ast.children:
#       (resN, finished) = nthHelper(child, id, kind, finished)
#       res += resN
#     return (res, finished)

# func nth* (ast: BrackNode, id: string): int =
#   # TODO: 展開済みマクロがあるのでうまく動作しない
#   let kind = ast.find(id).kind
#   let res = nthHelper(ast, id, kind, false)
#   result = res.n

func count* (ast: BrackNode, kind: BrackNodeKind, name: string, parentKind = bnkInvalid): int =
  if ast.kind == bnkIdent and kind == parentKind and ast.val == name:
    return 1
  elif ast.hasChildren:
    for child in ast.children:
      result += count(child, kind, name, ast.kind)
    if ast.kind == bnkRoot:
      result += 1

proc deleteHelper (ast: BrackNode, id: string) =
  if ast.hasChildren:
    var newAst = ast.kind.newTree(ast.id, @[])
    for child in ast.children:
      deleteHelper(child, id)
      if child.id != ast.id:
        newAst.children.add child

proc delete* (ast: BrackNode, id: string) =
  if ast.hasChildren:
    deleteHelper(ast, id)
  else:
    raise newException(BrackNoChildrenError, noChildrenErrorMsg(ast))

proc delete* (ast: BrackNode, index: int) =
  if ast.hasChildren:
    var newAst = ast.kind.newTree(ast.id)
    for i, child in ast.children:
      if i != index:
        newAst.children.add child
    ast[].children = newAst[].children
  else:
    raise newException(BrackNoChildrenError, noChildrenErrorMsg(ast))

proc insertHelper (ast: BrackNode, id: string, insertAst: BrackNode) =
  if ast.hasChildren:
    var newAst = ast.kind.newTree(ast.id, @[])
    for child in ast.children:
      child.insertHelper(id, insertAst)
      newAst.children.add child
      if child.id == id:
        newAst.children.add insertAst
    ast[].children = newAst[].children

proc insert* (ast: BrackNode, id: string, insertAst: BrackNode) =
  if ast.hasChildren:
    insertHelper(ast, id, insertAst)
  else:
    raise newException(BrackNoChildrenError, noChildrenErrorMsg(ast))

proc add* (ast, insertAst: BrackNode) =
  if ast.hasChildren:
    ast[].children.add insertAst
  else:
    raise newException(BrackNoChildrenError, noChildrenErrorMsg(ast))

func exists* (ast: BrackNode, id: string): bool =
  if ast.kind == bnkText or ast.kind == bnkIdent:
    return ast.id == id
  else:
    var res = ast.id == id
    for child in ast.children:
      res = res or exists(child, id)
    return res

# proc assignHelper (ast: BrackNode, id: string, node: BrackNode): BrackNode =
#   if ast.id == id:
#     var node = node
#     node.id = id
#     return node
#   elif ast.kind == bnkText or ast.kind == bnkIdent:
#     return ast
#   else:
#     var parent = BrackNode(kind: ast.kind, id: ast.id)
#     for child in ast.children:
#       parent.children.add assignHelper(child, id, node)
#     return parent

# type
#   DontHaveChildrenError* = object of ValueError
#   DontHaveValError* = object of ValueError

# proc `[]`* (ast: BrackNode, index: int): var BrackNode =
#   if ast.hasChildren:
#     result = ast.children[index]
#   else:
#     raise newException(DontHaveChildrenError, &"{ast.id} doesn't have children")

func id* (ast: BrackNode): string =
  result = ast.id

func kind* (ast: BrackNode): BrackNodeKind =
  result = ast.kind

func val* (ast: BrackNode): string =
  if ast.hasChildren:
    raise newException(Defect, &"{ast.id} doesn't have val")
  else:
    result = ast.val

func children* (ast: BrackNode): seq[BrackNode] =
  if ast.hasChildren:
    result = ast.children
  else:
    raise newException(Defect, &"{ast.id} doesn't have children")

proc `children=`* (ast: BrackNode, children: seq[BrackNode]) =
  if ast.hasChildren:
    ast[].children = children
  else:
    raise newException(Defect, &"{ast.id} doesn't have children")

# proc `val=`* (ast: var BrackNode, val: string) =
#   if ast.hasChildren:
#     raise newException(DontHaveValError, &"{ast.id} doesn't have val")
#   else:
#     ast.val = val

# proc `[]=`* (ast: var BrackNode, id: string, node: BrackNode) =
#   ast = assignHelper(ast, id, node)

# import std/macros

# macro quote* (body: untyped): untyped =
#   echo body.astGenRepr
#   result = nnkStmtList.newTree(
#     nnkCall.newTree(
#       macros.newIdentNode("BrackNode")
#     )
#   )