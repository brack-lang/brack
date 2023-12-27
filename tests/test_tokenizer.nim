import std/unittest
import brack/tokenizer
from brack/types/tokenizer as ttk import TokenKind

test "split no commands":
  let tokens = tokenize("Hello, World!")
  check tokens == [initToken(tkText, "Hello, World!")]

test "split commands":
  let tokens = tokenize("Hello, [:]")
  check tokens == [
    initToken(tkText, "Hello, "),
    initToken(tkSquareBracketOpen, "["),
    initToken(tkText, ":"),
    initToken(tkSquareBracketClose, "]")
  ]

test "split commands with an argument includes square brackets":
  let tokens = tokenize("Hello, [* World!]")
  check tokens == [
    initToken(tkText, "Hello, "),
    initToken(tkSquareBracketOpen, "["),
    initToken(tkText, "*"),
    initToken(tkText, "World!"),
    initToken(tkSquareBracketClose, "]")
  ]

test "split commands with an argument includes curly brackets":
  let tokens = tokenize("Hello, {* World!}")
  check tokens == [
    initToken(tkText, "Hello, "),
    initToken(tkCurlyBracketOpen, "{"),
    initToken(tkText, "*"),
    initToken(tkText, "World!"),
    initToken(tkCurlyBracketClose, "}")
  ]

test "split commands with an argument includes angle brackets":
  let tokens = tokenize("Hello, <* World!>")
  check tokens == [
    initToken(tkText, "Hello, "),
    initToken(tkAngleBracketOpen, "<"),
    initToken(tkText, "*"),
    initToken(tkText, "World!"),
    initToken(tkAngleBracketClose, ">")
  ]

test "split commands with two arguments includes square brackets":
  let tokens = tokenize("Hello, [@ World!, https://example.com/]")
  check tokens == [
    initToken(tkText, "Hello, "),
    initToken(tkSquareBracketOpen, "["),
    initToken(tkText, "@"),
    initToken(tkText, "World!"),
    initToken(tkComma, ","),
    initToken(tkText, "https://example.com/"),
    initToken(tkSquareBracketClose, "]")
  ]

test "split nesting commands":
  let tokens = tokenize("Hello, [* [@ World!, https://example.com/]]")
  check tokens == [
    initToken(tkText, "Hello, "),
    initToken(tkSquareBracketOpen, "["),
    initToken(tkText, "*"),
    initToken(tkSquareBracketOpen, "["),
    initToken(tkText, "@"),
    initToken(tkText, "World!"),
    initToken(tkComma, ","),
    initToken(tkText, "https://example.com/"),
    initToken(tkSquareBracketClose, "]"),
    initToken(tkSquareBracketClose, "]")
  ]

test "include newlines":
  let tokens = tokenize("""Hello,
World,
{** Contact}
[@ My website, https://example.com/]

2023.11.30
""")
  check tokens == [initToken(tkText, "Hello,")]