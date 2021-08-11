#ifndef AST_HH
#define AST_HH

#include "token.hh"
#include "libraries.hh"

enum class ASTNodeType: int
{
    Program,
    Integer
};


class ASTNode
{
public:
    ASTNodeType type;
};


class Expression: public ASTNode {};


class Program: public ASTNode
{
public:
    std::vector<Expression* > expressions;

    Program();
};


class Integer: public Expression
{
public:
    Integer(Token token);
    long long value;
};

#endif
