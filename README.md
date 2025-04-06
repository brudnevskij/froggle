# Froggle

Froggle is a tiny toy programming language written in Rust as part of a compiler construction course.

## ✨ Initial Features

- Integer arithmetic (`+`, `-`, `*`, `/`)
- Variable assignments
- Print statement

## 🔣 Grammar (BNF)

```bnf
<program> ::= <statement_list>

<statement_list> ::= <statement>
                   | <statement> ";" <statement_list>

<statement> ::= <assignment>
              | <print>

<assignment> ::= "let" <identifier> "=" <expression>

<print> ::= "croak" <expression>

<expression> ::= <term>
               | <expression> "+" <term>
               | <expression> "-" <term>

<term> ::= <factor>
         | <term> "*" <factor>
         | <term> "/" <factor>

<factor> ::= <number>
           | <identifier>
           | "(" <expression> ")"

<identifier> ::= <letter> { <letter> | <digit> }
<number> ::= <digit> { <digit> }

<letter> ::= "a" | ... | "z" | "A" | ... | "Z"
<digit> ::= "0" | ... | "9"
```