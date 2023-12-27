import brack/tokenizer
import brack/parser
import brack/ast
import fusion/matching
import std/unittest
import std/options

test "split no commands":
    let res = tokenize("Hello, World!").parse
    check res.isSome and (let ast = res.get; ast ~= bnkDocument.newTree(
        bnkStmt.newTree(
            bnkExprSeq.newTree(
                bnkExpr.newTree(
                    newTextNode("Hello, World!")
                )
            )
        )
    ))

test "split commands":
    check if Some(@ast) ?= tokenize("Hello, [:]").parse:
        ast ~= bnkDocument.newTree(
            bnkStmt.newTree(
                bnkExprSeq.newTree(
                    bnkExpr.newTree(
                        newTextNode("Hello, "),
                        bnkSquareBracket.newTree(
                            bnkCommandSpec.newTree(
                                newTextNode(":")
                            )
                        )
                    )
                )
            )
        )
    else: false

test "split commands with an argument includes square brackets":
    check if Some(@ast) ?= tokenize("Hello, [* World!]").parse:
        echo ast
        ast ~= bnkDocument.newTree(
            bnkStmt.newTree(
                bnkExprSeq.newTree(
                    bnkExpr.newTree(
                        newTextNode("Hello, "),
                        bnkSquareBracket.newTree(
                            bnkCommandSpec.newTree(
                                newTextNode("*")
                            ),
                            bnkArguments.newTree(
                                newTextNode("World!")
                            )
                        )
                    )
                )
            )
        )
    else: false
