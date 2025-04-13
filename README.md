# Froggle

Froggle is a tiny toy programming language written in Rust as part of a compiler construction course.

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