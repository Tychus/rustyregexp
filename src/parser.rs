/* Regex grammar : Backus-Naur form (BNF)
    Regex ::= concat '|' Regex | concat
    concat ::= loop '.' concat | loop
    loop ::= atom '*' | atom '+' | atom
    atom ::= '[' Regex ']' | char
    char ::= alnum char
*/

// parse char

// parse atom

// parse loop

// parse concat

// parse regex