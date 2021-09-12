#include "evaluator.hh"
#include "object.hh"
#include "ast.hh"


Object::Object* Evaluator::evaluate(ASTNode *node) {
    if (node == nullptr)
        return nullptr;
    
    switch (node->type) {
        case ASTNodeType::Program: {
            Program* program = static_cast<Program*>(node);
            return evaluate_program(program);
        } break;
        case ASTNodeType::Assignment: {
            Assignment* assign = static_cast<Assignment*>(node);
            env[assign->getName()] = evaluate(assign->getValue());
        } break;
        case ASTNodeType::Boolean: {
            Boolean* newBool = static_cast<Boolean*>(node);
            return new Object::Boolean(newBool->value);
        } break;
        case ASTNodeType::Identifier: {
            Identifier* ident = static_cast<Identifier*>(node);
            auto it = env.find(ident->getName());
            if (it != env.end())
                return it->second;
            else
                return nullptr; // error? idk
        } break;
        case ASTNodeType::Infix: {
            Infix* infix = static_cast<Infix*>(node);
            return evaluate_infix(infix);
        } break;
        case ASTNodeType::Integer: {
            Integer* newInt = static_cast<Integer*>(node);
            return new Object::Integer(newInt->value);
        } break;
        default:
            break;
    }

    return nullptr;
}


Object::Object* Evaluator::evaluate_infix(Infix *infix) {
    if (infix->getLeft()->type != infix->getRight()->type)
        return nullptr; // error (for now)
    
    ASTNodeType operand_type = infix->getLeft()->type;
    switch (infix->getOp().type) {
        case TokenType::PLUS:
            if (operand_type == ASTNodeType::Integer)
                return new Object::Integer(
                    static_cast<Integer*>(infix->getLeft())->value +
                    static_cast<Integer*>(infix->getRight())->value
                );
            break;
        case TokenType::MINUS:
            if (operand_type == ASTNodeType::Integer)
                return new Object::Integer(
                    static_cast<Integer*>(infix->getLeft())->value -
                    static_cast<Integer*>(infix->getRight())->value
                );
            break;
        default:
            break;
    }

    return nullptr; // error
}


Object::Object* Evaluator::evaluate_program(Program* program) {
    Object::Object* res = nullptr;

    for (int i = 0; i < program->expressions.size(); i++) {
        res = evaluate(program->expressions[i]);
    }

    return res;
}
