#include "evaluator.hh"
#include "object.hh"
#include "ast.hh"


Object::Object* Evaluator::evaluate(
    ASTNode *node,
    std::unordered_map<std::wstring, Object::Object*> &env
    ) {
    if (node == nullptr)
        return nullptr;
    
    switch (node->type) {
        case ASTNodeType::Assignment: {
            Assignment* assign = static_cast<Assignment*>(node);
            env[assign->getName()] = evaluate(assign->getValue(), env);
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
                return nullptr; // error (for now)
        } break;
        case ASTNodeType::If: {
            If *if_node = static_cast<If*>(node);
            return evaluate_if(if_node, env);
        } break;
        case ASTNodeType::Infix: {
            Infix* infix = static_cast<Infix*>(node);
            return evaluate_infix(infix, env);
        } break;
        case ASTNodeType::Integer: {
            Integer* newInt = static_cast<Integer*>(node);
            return new Object::Integer(newInt->value);
        } break;
        case ASTNodeType::Minus: {
            Minus* minus = static_cast<Minus*>(node);
            return evaluate_minus(minus, env);
        } break;
        case ASTNodeType::Not: {
            Not* no = static_cast<Not*>(node);
            return evaluate_not(no, env);
        } break;
        case ASTNodeType::While: {
            While* while_node = static_cast<While*>(node);
            return evaluate_while(while_node, env);
        } break;
        case ASTNodeType::Function: {
            Function* function = static_cast<Function*>(node);
            return evaluate_function(function);
        } break;
        case ASTNodeType::FunctionCall: {
            FunctionCall* call = static_cast<FunctionCall*>(node);
            return evaluate_call(call, env);
        } break;
        default:
            break;
    }

    return nullptr;
}


Object::Object* Evaluator::evaluate_block(Block *block, std::unordered_map<std::wstring, Object::Object*> &env) {
    Object::Object *res = nullptr;
    for (int i = 0; i < block->routine.size(); i++)
        res = evaluate(block->routine[i], env);
    
    return res;
}


Object::Object* Evaluator::evaluate_call(FunctionCall* call, std::unordered_map<std::wstring, Object::Object*> &env) {    
    Object::Object *temp = evaluate(new Identifier(call->getName()), env);
    if (!(temp != nullptr && temp->type == Object::ObjectType::Function))
        return nullptr; // error
    Object::Function *func = static_cast<Object::Function*>(temp);
    
    std::vector<ASTNode*> names = static_cast<Function*>(func->getFunction())->getParams();
    std::vector<ASTNode*> params = call->getParams();
    if (names.size() != params.size())
        return nullptr; // error
    
    std::unordered_map<std::wstring, Object::Object*> new_env;
    for (int i = 0; i < (int)names.size(); i++)
        new_env[static_cast<Identifier*>(names[i])->getName()] = evaluate(params[i], env);
    
    return evaluate_block(
        static_cast<Function*>(func->getFunction())->getRoutine(),
        new_env
    );
}


Object::Object* Evaluator::evaluate_function(Function* function) {
    return new Object::Function(function);
}


Object::Object* Evaluator::evaluate_if(If *if_node, std::unordered_map<std::wstring, Object::Object*> &env) {
    Object::Object *condition = evaluate(if_node->getCondition(), env);
    if (condition->type != Object::ObjectType::Boolean)
        return nullptr;
    
    bool value = static_cast<Object::Boolean*>(condition)->value;
    ASTNode *res = value? if_node->getConsequence() : if_node->getAlternative();
    
    return evaluate_block(static_cast<Block*>(res), env);
}


Object::Object* Evaluator::evaluate_infix(Infix *infix, std::unordered_map<std::wstring, Object::Object*> &env) {
    Object::Object *left = evaluate(infix->getLeft(), env);
    Object::Object *right = evaluate(infix->getRight(), env);
    if (left == nullptr || right == nullptr)
        return nullptr; // error
    if (left->type != right->type)
        return nullptr; // error (for now)
    
    Object::ObjectType operand_type = left->type;
    switch (infix->getOp().type) {
        case TokenType::EQUALS:
            if (operand_type == Object::ObjectType::Integer)
                return new Object::Boolean(
                    static_cast<Object::Integer*>(left)->value ==
                    static_cast<Object::Integer*>(right)->value
                );
            else if (operand_type == Object::ObjectType::Boolean)
                return new Object::Boolean(
                    static_cast<Object::Boolean*>(left)->value ==
                    static_cast<Object::Boolean*>(right)->value
                );
            break;
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


Object::Object* Evaluator::evaluate_minus(Minus *minus, std::unordered_map<std::wstring, Object::Object*> &env) {
    if (minus->getValue() == nullptr)
        return nullptr; // error
    
    Object::Object* value = evaluate(minus->getValue(), env);
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


Object::Object* Evaluator::evaluate_not(Not *no, std::unordered_map<std::wstring, Object::Object*> &env) {
    if (no->getValue() == nullptr)
        return nullptr; // error
    
    Object::Object* value = evaluate(no->getValue(), env);
    switch (value->type) {
        case Object::ObjectType::Boolean:
            return new Object::Boolean(
                !static_cast<Object::Boolean*>(value)->value
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
        res = evaluate(program->ASTNodes[i], first_env);
    }

    return res;
}


Object::Object* Evaluator::evaluate_while(While *while_node, std::unordered_map<std::wstring, Object::Object*> &env) {
    Object::Object* res = nullptr;

    Object::Object* condition = evaluate(while_node->getCondition(), env);
    if (condition->type != Object::ObjectType::Boolean)
        return nullptr; // error


    while (static_cast<Object::Boolean*>(condition)->value) {
        res = evaluate_block(static_cast<Block*>(while_node->getRoutine()), env);

        condition = evaluate(while_node->getCondition(), env);
        if (condition->type != Object::ObjectType::Boolean)
            return nullptr; // error
    }

    return res;
}
