#include "evaluator.hh"
#include "object.hh"
#include "ast.hh"

Evaluator::Evaluator() {}


Object::Object* Evaluator::evaluate(ASTNode *node) {
    if (node == nullptr)
        return nullptr;
    
    switch (node->type) {
         case ASTNodeType::Program: {
            Program* program = static_cast<Program*>(node);
            return evaluateProgram(program);

        } break;
        case ASTNodeType::Integer: {
            Integer* newInt = static_cast<Integer*>(node);
            return new Object::Integer(newInt->value);
        } break;
    }

    return nullptr;
}


Object::Object* Evaluator::evaluateProgram(Program* program) {
    Object::Object* res = nullptr;

    for (int i = 0; i < program->expressions.size(); i++) {
        res = evaluate(program->expressions[i]);
    }

    return res;
}
