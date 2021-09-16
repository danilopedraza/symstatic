#ifndef EVALUATOR_HH
#define EVALUATOR_HH

#include "ast.hh"
#include "object.hh"

#include "libraries.hh"

class Evaluator
{
public:
    std::unordered_map<std::wstring, Object::Object*> env;

    Object::Object* evaluate(ASTNode *node);
    Object::Object* evaluate_block(Block *block);
    Object::Object* evaluate_if(If *if_node);
    Object::Object* evaluate_infix(Infix *infix);
    Object::Object* evaluate_minus(Minus *minus);
    Object::Object* evaluate_not(Not *no);
    Object::Object* evaluate_program(Program *program);
    Object::Object* evaluate_while(While *while_node);
};

#endif
