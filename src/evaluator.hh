#ifndef EVALUATOR_HH
#define EVALUATOR_HH

#include "ast.hh"
#include "object.hh"

class Evaluator
{
public:
    Object::Object* evaluate(ASTNode *node);
    Object::Object* evaluateProgram(Program *program);
};

#endif
