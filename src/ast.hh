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
    Function,
    FunctionCall,
    Identifier,
    If,
    Infix,
    Integer,
    Minus,
    Not,
    While
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


class Function: public ASTNode
{
private:
    Block *routine;
    std::vector<ASTNode*> params;
public:
    Function(std::vector<ASTNode*> params, Block *routine);
    Block *getRoutine();
    std::vector<ASTNode*> getParams();
};


class FunctionCall: public ASTNode
{
private:
    std::wstring name;
    std::vector<ASTNode*> params;
public:
    FunctionCall(std::wstring name, std::vector<ASTNode*> params);
    std::wstring getName();
    std::vector<ASTNode*> getParams();
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


class Minus: public ASTNode
{
private:
    ASTNode *value;
public:
    Minus(ASTNode *value);
    ASTNode *getValue();
};


class Not: public ASTNode
{
private:
    ASTNode *value;
public:
    Not(ASTNode *value);
    ASTNode *getValue();
};


class While: public ASTNode
{
private:
    ASTNode *condition;
    ASTNode *routine;
public:
    While(ASTNode *condition, ASTNode *routine);
    ASTNode *getCondition();
    ASTNode *getRoutine();
};

#endif
