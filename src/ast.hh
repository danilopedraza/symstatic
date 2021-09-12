#ifndef AST_HH
#define AST_HH

#include "token.hh"
#include "libraries.hh"

enum class ASTNodeType: int
{
    Assignment,
    Program,
    Boolean,
    Identifier,
    Infix,
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


class Assignment: public Expression
{
private:
    std::wstring name;
    Expression *value;
public:
    Assignment(std::wstring name, Expression *value);
    std::wstring getName();
    Expression *getValue();
    void setValue(Expression *value);
};


class Boolean: public Expression
{
public:
    Boolean(bool value);
    bool value;
};


class Identifier: public Expression
{
private:
    std::wstring name;
public:
    Identifier(std::wstring name);
    std::wstring getName();

};


class Infix: public Expression
{
private:
    Expression *left;
    Token op;
    Expression *right;
public:
    Infix(Expression *left, Token op, Expression *right);
    Expression *getLeft();
    Expression *getRight();
    Token getOp();
};


class Integer: public Expression
{
public:
    Integer(Token token);
    long long value;
};




#endif
