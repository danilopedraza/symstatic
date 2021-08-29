#ifndef TOKEN_HH
#define TOKEN_HH

#include "libraries.hh"

enum class TokenType: int
{
    EOFILE = -1,
    ASSIGN,
    IDENT,
    ILLEGAL,
    INT,
    TRUE,
    FALSE
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
