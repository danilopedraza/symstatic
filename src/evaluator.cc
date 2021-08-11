#include "evaluator.hh"
#include "object.hh"
#include "ast.hh"


Object::Object* Evaluator::evaluate(ASTNode *node) {
    if (node == nullptr)
        return nullptr;
    
    switch (node->type) {
        case ASTNodeType::Program: {
            Program* program = static_cast<Program*>(node);
            return evaluateProgram(program);
        } break;
        case ASTNodeType::Boolean: {
            Boolean* newBool = static_cast<Boolean*>(node);
            return new Object::Boolean(newBool->value);
        } break;
        case ASTNodeType::Integer: {
            Integer* newInt = static_cast<Integer*>(node);
            return new Object::Integer(newInt->value);
        } break;
        default:
            return nullptr;
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
