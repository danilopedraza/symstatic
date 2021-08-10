#ifndef TOKEN_HH
#define TOKEN_HH

#include "libraries.hh"

enum class TokenType: int
{
    EOFILE = -1,
    ILLEGAL,
    INT
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
