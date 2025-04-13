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
- [ ] Bottom-up parser (not required for your current scope)
- [ ] Nested scopes (e.g., block-local variables)
- [x] Static types (`let x: number = ...`)
- [x] Type checker with compile-time type errors
- [ ] Transpile AST to Python or Java

## ✨ Initial Features

- Integer arithmetic (`+`, `-`, `*`, `/`)
- Boolean operators (`==`, `>`, `<`)
- Variable assignments
- Print statement

## 🔣 Grammar (BNF)

```bnf
<program> ::= <statement_list>

<statement_list> ::= <statement>
                   | <statement> ";" <statement_list>

<statement> ::= <assignment>
              | <print>
              | <while>

<assignment> ::= "let" <identifier> ":" <type> "=" <expression>

<print> ::= "croak" <expression>

<while> ::= "while" <expression> "{" <statement_list> "}"

<expression> ::= <term>
               | <expression> "+" <term>
               | <expression> "-" <term>
               | <expression>  "==" <term>
               | <expression>  ">" <term>
               | <expression>  "<" <term>               

<term> ::= <factor>
         | <term> "*" <factor>
         | <term> "/" <factor>

<factor> ::= <number>
           | <identifier>
           | <bool>
           | "(" <expression> ")"

<identifier> ::= <letter> { <letter> | <digit> }
<number> ::= <digit> { <digit> }
<bool> ::= "true" | "false"

<letter> ::= "a" | ... | "z" | "A" | ... | "Z"
<digit> ::= "0" | ... | "9"
<type> ::= "number" | "bool"
```