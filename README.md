<div align="center">
  <img width="998" alt="banner" src="https://github.com/user-attachments/assets/8ff9db18-19cd-47f0-9b7c-e5ca745174a4">
  <br />
  <b>🚀 Rust based recursive descent parser and interpreter for the <em>Lox</em> programming language 📦</b>
  <br />
</div>

# Roxi
CLI lexer, parser and interpreter for __Lox.__ This challenge began as part of the Codecrafters' ["Build Your Own Build your own Interpreter" Challenge](https://app.codecrafters.io/courses/interpreter/overview), and has since been continued by me, closely following the book [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Installation and Usage:
**Requirements:**
This project is based on `cargo >= 1.77`, so make sure you have at least that installed.

**Get the project:**
`git clone github.com/mal0ner/roxi.git`

**Usage:**
Tokenize a lox file:
```sh
# test.lox
# fun fibonacci(n) {
#   if (n <= 1) return n;
#   return fibonacci(n - 2) + fibonacci(n - 1);
# }

$ cargo run tokenize test.lox

# OUTPUT
# IDENTIFIER fibonacci null
# LEFT_PAREN ( null
# IDENTIFIER n null
# MINUS - null
# NUMBER 2 2.0
# RIGHT_PAREN ) null
# PLUS + null
# IDENTIFIER fibonacci null
# LEFT_PAREN ( null
# IDENTIFIER n null
# MINUS - null
# NUMBER 1 1.0
# RIGHT_PAREN ) null
# SEMICOLON ; null
# RIGHT_BRACE } null
# EOF  null
```

Parse a lox file:
```sh
# ((10 + 5) * 3 - 4 / 2 == 42) != true == !false == 7 >= 6 <= 5 > 4 < 3 + -1 * "hello" == nil ==
# (100 - 50) * 2 / (4 + 1) >= 10 == ("world" == "world") != false ==
# !(5 < 3) == true == 7 * (3 + 2) - 1 == 34 ==
# (nil == nil) != (10 != 10) == 
# -(-(-10)) + 20 / 4 * 2 - 15 == -10 ==
# "concatenation" + " " + "works" == "concatenation works" ==
# !!!true == false ==
# (((1 + 2) * 3 - 4) / 5 + 6 - 7) * 8 == 32 ==
# (9 >= 8 >= 7 >= 6 >= 5) == true ==
# "nested" == ("groups" == ("are" == ("fun" == "fun"))) ==
# (1 < 2 > 3 < 4 > 5) == false ==
# nil != nil == (true != false) ==
# (123.45 + 67.89) * (10.5 - 5.5) == 955.7 ==
# !(!(!(false))) == true ==
# ("a" + "b" + "c") * 3 == "abcabcabc" ==
# ((((1 + 1) + 1) + 1) + 1) == 5

$ cargo run parse test.lox

# OUTPUT
# (== (== (== (== (== (== (== (== (== (!= (== (== (== (== (== (== (== (== (== (== (== (== (== (== (== (!= (== (== (== (== (== (!= (== (== (== (== (== (!= # (group (== (- (* (group (+ 10.0 5.0)) 3.0) (/ 4.0 2.0)) 42.0)) true) (! false)) (< (> (<= (>= 7.0 6.0) 5.0) 4.0) (+ 3.0 (* (- 1.0) hello)))) nil) (>= (/ # (* (group (- 100.0 50.0)) 2.0) (group (+ 4.0 1.0))) 10.0)) (group (== world world))) false) (! (group (< 5.0 3.0)))) true) (- (* 7.0 (group (+ 3.0 
# 2.0))) 1.0)) 34.0) (group (== nil nil))) (group (!= 10.0 10.0))) (- (+ (- (group (- (group (- 10.0))))) (* (/ 20.0 4.0) 2.0)) 15.0)) (- 10.0)) (+ (+   
# concatenation  ) works)) concatenation works) (! (! (! true)))) false) (* (group (- (+ (/ (group (- (* (group (+ 1.0 2.0)) 3.0) 4.0)) 5.0) 6.0) 7.0))   # 8.0)) 32.0) (group (>= (>= (>= (>= 9.0 8.0) 7.0) 6.0) 5.0))) true) nested) (group (== groups (group (== are (group (== fun fun))))))) (group (> (< (> (< # 1.0 2.0) 3.0) 4.0) 5.0))) false) nil) nil) (group (!= true false))) (* (group (+ 123.45 67.89)) (group (- 10.5 5.5)))) 955.7) (! (group (! (group (!     # (group false))))))) true) (* (group (+ (+ a b) c)) 3.0)) abcabcabc) (group (+ (group (+ (group (+ (group (+ 1.0 1.0)) 1.0)) 1.0)) 1.0))) 5.0)
```

```sh
# test.lox
# ("a" + "pple") == "apple"

$ cargo run evaluate t.lox

# OUTPUT
true
```

## Working Features
### Tokenizer
  - [x] Literals
  - [x] Operators
  - [x] Keywords
  - [x] Identifiers

### Parser
  - [x] Basic Expressions
  - [ ] Statements (WIP)
  - [ ] Control Flow
  - [ ] Functions
  - [ ] Classes

### Evaluator
  - [x] Boolean expressions
  - [x] Numeric expressions
  - [x] String concatenation

## Coming Soon...
- Environment and State
- Value binding and resolution
- Classes and OOP features
- Improved error reporting

Obviously this is not yet a fully functional implementation of Lox, but I have learned so much about rust and programming language design and the rust language itself from only what's been completed so far. This was my original goal in tackling this project and I am happy with the turnout.
