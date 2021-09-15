#include "evaluator.hh"
#include "object.hh"
#include "ast.hh"


Object::Object* Evaluator::evaluate(ASTNode *node) {
    if (node == nullptr)
        return nullptr;
    
    switch (node->type) {
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
        case ASTNodeType::If: {
            If *if_node = static_cast<If*>(node);
            return evaluate_if(if_node);
        } break;
        case ASTNodeType::Infix: {
            Infix* infix = static_cast<Infix*>(node);
            return evaluate_infix(infix);
        } break;
        case ASTNodeType::Integer: {
            Integer* newInt = static_cast<Integer*>(node);
            return new Object::Integer(newInt->value);
        } break;
        case ASTNodeType::Minus: {
            Minus* minus = static_cast<Minus*>(node);
            return evaluate_minus(minus);
        } break;
        case ASTNodeType::Program: {
            Program* program = static_cast<Program*>(node);
            return evaluate_program(program);
        } break;
        default:
            break;
    }

    return nullptr;
}


Object::Object* Evaluator::evaluate_block(Block *block) {
    Object::Object *res = nullptr;
    for (int i = 0; i < block->routine.size(); i++)
        res = evaluate(block->routine[i]);
    
    return res;
}


Object::Object* Evaluator::evaluate_if(If *if_node) {
    Object::Object *condition = evaluate(if_node->getCondition());
    if (condition->type != Object::ObjectType::Boolean)
        return nullptr;
    
    bool value = static_cast<Object::Boolean*>(condition)->value;
    ASTNode *res = value? if_node->getConsequence() : if_node->getAlternative();
    
    return evaluate_block(static_cast<Block*>(res));
}


Object::Object* Evaluator::evaluate_infix(Infix *infix) {
    Object::Object *left = evaluate(infix->getLeft());
    Object::Object *right = evaluate(infix->getRight());
    if (left == nullptr || right == nullptr)
        return nullptr; // error
    if (left->type != right->type)
        return nullptr; // error (for now)
    
    Object::ObjectType operand_type = left->type;
    switch (infix->getOp().type) {
        case TokenType::PLUS:
            if (operand_type == Object::ObjectType::Integer)
                return new Object::Integer(
                    static_cast<Object::Integer*>(left)->value +
                    static_cast<Object::Integer*>(right)->value
                );
            break;
        case TokenType::MINUS:
            if (operand_type == Object::ObjectType::Integer)
                return new Object::Integer(
                    static_cast<Object::Integer*>(left)->value -
                    static_cast<Object::Integer*>(right)->value
                );
            break;
        case TokenType::MULTIPLICATION:
            if (operand_type == Object::ObjectType::Integer)
                return new Object::Integer(
                    static_cast<Object::Integer*>(left)->value *
                    static_cast<Object::Integer*>(right)->value
                );
            break;
        case TokenType::DIVISION:
            if (operand_type == Object::ObjectType::Integer)
                return new Object::Integer(
                    static_cast<Object::Integer*>(left)->value /
                    static_cast<Object::Integer*>(right)->value
                );
            break;
        default:
            break;
    }

    return nullptr; // error
}


Object::Object* Evaluator::evaluate_minus(Minus *minus) {
    if (minus->getValue() == nullptr)
        return nullptr; // error
    
    Object::Object* value = evaluate(minus->getValue());
    switch (value->type) {
        case Object::ObjectType::Integer:
            return new Object::Integer(
                -static_cast<Object::Integer*>(value)->value
            );
            break;
        default:
            break;
    }

    return nullptr; // error
}


Object::Object* Evaluator::evaluate_program(Program* program) {
    Object::Object* res = nullptr;

    for (int i = 0; i < program->ASTNodes.size(); i++) {
        res = evaluate(program->ASTNodes[i]);
    }

    return res;
}
