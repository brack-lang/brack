import unittest
import brack/tokenizer

test "split no commands":
  let tokenizer = newTokenizer()
  let tokens = tokenizer.tokenize("Hello, World!")
  check tokens == ["Hello, World!"]

test "split commands":
  let tokenizer = newTokenizer()
  let tokens = tokenizer.tokenize("Hello, [:]")
  check tokens == ["Hello, ", "[", ":", "]"]

test "split commands with an argument includes square brackets":
  let tokenizer = newTokenizer()
  let tokens = tokenizer.tokenize("Hello, [* World!]")
  check tokens == ["Hello, ", "[", "*", "World!", "]"]

test "split commands with an argument includes curly brackets":
  let tokenizer = newTokenizer()
  let tokens = tokenizer.tokenize("Hello, {* World!}")
  check tokens == ["Hello, ", "{", "*", "World!", "}"]

test "split commands with an argument includes angle brackets":
  let tokenizer = newTokenizer()
  let tokens = tokenizer.tokenize("Hello, <* World!>")
  check tokens == ["Hello, ", "<", "*", "World!", ">"]

test "split commands with two arguments includes square brackets":
  let tokenizer = newTokenizer()
  let tokens = tokenizer.tokenize("Hello, [@ World!, https://example.com/]")
  check tokens == ["Hello, ", "[", "@", "World!", ",", "https://example.com/", "]"]

test "split nesting commands":
  let tokenizer = newTokenizer()
  let tokens = tokenizer.tokenize("Hello, [* [@ World!, https://example.com/]]")
  check tokens == ["Hello, ", "[", "*", "[", "@", "World!", ",", "https://example.com/", "]", "]"]
