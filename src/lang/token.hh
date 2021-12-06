#ifndef TOKEN_HH
#define TOKEN_HH

#include "libraries.hh"

enum class TokenType: int
{
    EOFILE = -1,
    AND,
    ASSIGN,
    COMMA,
    DIVISION,
    DO,
    FUNCTION,
    ELSE,
    EQUALS,
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
    THEN,
    TRUE,
    RBRACE,
    RPAREN,
    SEMICOLON,
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
