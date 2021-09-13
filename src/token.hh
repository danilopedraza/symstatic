#ifndef TOKEN_HH
#define TOKEN_HH

#include "libraries.hh"

enum class TokenType: int
{
    EOFILE = -1,
    AND,
    ASSIGN,
    DIVISION,
    DO,
    FALSE,
    FOR,
    IDENT,
    IF,
    ILLEGAL,
    INT,
    LBRACE,
    LPAREN,
    MINUS,
    MULTIPLICATION,
    NOT,
    OR,
    POINT,
    PLUS,
    TRUE,
    RBRACE,
    RPAREN,
    WHILE
};


class Token
{
public:
    Token();

    TokenType type;
    std::wstring literal;
    int line, column;
};

#endif
