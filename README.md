# Froggle

Froggle is a tiny toy programming language written in Rust as part of a compiler construction course.


# ✅ Froggle Feature Checklist (Course Requirements)

## 🟩 Minimum for a Passing Grade (1)

- [x] BNF grammar
- [x] Lexer (tokenizer)
- [x] Ad-hoc parser
- [x] Expression evaluation with variables (e.g., `a + 6`)
- [x] Interpreter that runs the program
- [x] Global state (variable environment)
- [x] `let` statement for variable assignment
- [x] `croak` statement for printing
- [x] `while` loops

## ⭐️ Bonus Features (higher grades)

- [x] Error reporting (e.g., type mismatches, unknown variables)
- [x] Top-down parser with operator precedence
- [x] Nested scopes (e.g., block-local variables)
- [x] Static types (`let x: number = ...`)
- [x] Type checker with compile-time type error
- [x] Type inference at compile-time
- [x] Function declarations and calls
- [x] Return values from functions
- [x] If and if-else control flow
- [x] Expression statement evaluation
- [x] REPL and file execution modes
- [x] Separated type checker using visitor pattern

## ✨ Operators

- Integer arithmetic (`+`, `-`, `*`, `/`)
- Boolean operators (`==`, `>`, `<`)
- Variable assignments
- Print statement
- Block statements

## Installation
Assuming you have Rust installed, build project:
```shell
cargo build --release
```

Add binary to PATH:
```shell
export PATH="$PATH:$(pwd)/target/release"
```
Run:
```shell
froggle ./source_file.frog
# or run the REPL
froggle
```
There are five demo programs in the demo-programs dir.

## Grammar (BNF)

```bnf
<program> ::= <statement_list>

<statement_list> ::= <statement>
                   | <statement> ";" <statement_list>

<statement> ::= <declaration>
              | <print>
              | <while>
              | <assignment>
              | <block>
              | <function_decl>
              | <return>
              | <if>
              | <expression_statement>

<declaration> ::= "let" <identifier> { ":" <type> } "=" <expression>

<print> ::= "croak" <expression>

<while> ::= "while" <expression> "{" <statement_list> "}"

<assignment> ::= <identifier> "=" <expression>

<block> ::= "{" <statement_list> "}"

<function_decl> ::= "func" <identifier> "(" [<param_list>] ")" ":" <type> <block>

<param_list> ::= <identifier> ":" <type> { "," <identifier> ":" <type> }

<function_call> ::= <identifier> "(" [<arg_list>] ")"

<arg_list> ::= <expression> { "," <expression> }

<return> ::= "return" <expression>

<if> ::= "if" <expression> <statement> [ "else" <statement> ]

<expression_statement> ::= <expression>

<expression> ::= <term>
               | <term> "*" <term>
               | <term> "/" <term>
               | <expression> "+" <term>
               | <expression> "-" <term>
               | <expression>  "==" <term>
               | <expression>  ">" <term>
               | <expression>  "<" <term>
               | <function_call>

<term> ::= <term>
         | <number>
         | <identifier>
         | <bool>
         | "(" <expression> ")"

<identifier> ::= <letter> { <letter> | <digit> }
<number> ::= <digit> { <digit> }
<bool> ::= "true" | "false"

<letter> ::= "a" | ... | "z" | "A" | ... | "Z"
<digit> ::= "0" | ... | "9"
<type> ::= "number" | "bool" | "void"
```
