#ifndef AST_HH
#define AST_HH

#include "token.hh"
#include "libraries.hh"

enum class ASTNodeType: int
{
    Assignment,
    Block,
    Program,
    Boolean,
    Identifier,
    If,
    Infix,
    Integer
};


class ASTNode
{
public:
    ASTNodeType type;
};



class Program: public ASTNode
{
public:
    std::vector<ASTNode* > ASTNodes;

    Program();
};


class Assignment: public ASTNode
{
private:
    std::wstring name;
    ASTNode *value;
public:
    Assignment(std::wstring name, ASTNode *value);
    std::wstring getName();
    ASTNode *getValue();
    void setValue(ASTNode *value);
};


class Boolean: public ASTNode
{
public:
    Boolean(bool value);
    bool value;
};


class Block: public ASTNode
{
public:
    Block();
    std::vector<ASTNode* > routine;
};


class Identifier: public ASTNode
{
private:
    std::wstring name;
public:
    Identifier(std::wstring name);
    std::wstring getName();

};


class If: public ASTNode
{
private:
    ASTNode *condition;
    ASTNode *consequence;
    ASTNode *alternative;
public:
    If(ASTNode *condition, ASTNode *consequence, ASTNode *alternative);
    ASTNode* getCondition();
    ASTNode* getConsequence();
    ASTNode* getAlternative();
};


class Infix: public ASTNode
{
private:
    ASTNode *left;
    Token op;
    ASTNode *right;
public:
    Infix(ASTNode *left, Token op, ASTNode *right);
    ASTNode *getLeft();
    ASTNode *getRight();
    Token getOp();
};


class Integer: public ASTNode
{
public:
    Integer(Token token);
    long long value;
};




#endif
