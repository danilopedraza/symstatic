#ifndef EVALUATOR_HH
#define EVALUATOR_HH

#include "ast.hh"
#include "object.hh"

#include "libraries.hh"

class Evaluator
{
public:
    std::unordered_map<std::wstring, Object::Object*> first_env;

    Object::Object* evaluate(ASTNode *node, std::unordered_map<std::wstring, Object::Object*> &env);
    Object::Object* evaluate_block(Block *block, std::unordered_map<std::wstring, Object::Object*> &env);
    Object::Object* evaluate_call(FunctionCall* call, std::unordered_map<std::wstring, Object::Object*> &env);
    Object::Object* evaluate_function(Function* function);
    Object::Object* evaluate_if(If *if_node, std::unordered_map<std::wstring, Object::Object*> &env);
    Object::Object* evaluate_infix(Infix *infix, std::unordered_map<std::wstring, Object::Object*> &env);
    Object::Object* evaluate_minus(Minus *minus, std::unordered_map<std::wstring, Object::Object*> &env);
    Object::Object* evaluate_not(Not *no, std::unordered_map<std::wstring, Object::Object*> &env);
    Object::Object* evaluate_program(Program *program);
    Object::Object* evaluate_while(While *while_node, std::unordered_map<std::wstring, Object::Object*> &env);
};

#endif
