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
    Object::Object* evaluate_infix(Infix *infix);
    Object::Object* evaluate_program(Program *program);
};

#endif
