#include "ast.hh"
#include "libraries.hh"

Program::Program() {
    type = ASTNodeType::Program;
    expressions = std::vector<Expression* >();
}


Assignment::Assignment(std::wstring name, Expression *value) {
    type = ASTNodeType::Assignment;
    this->name = name;
    this->value = value;
}
std::wstring Assignment::getName() {return name;}
Expression* Assignment::getValue() {return value;}
void Assignment::setValue(Expression *value) {this->value = value;}


Boolean::Boolean(bool value) {
    type = ASTNodeType::Boolean;
    this->value = value;
}


Block::Block() {
    type = ASTNodeType::Block;
}


Identifier::Identifier(std::wstring name) {
    type = ASTNodeType::Identifier;
    this->name = name;
}
std::wstring Identifier::getName() {return name;}


Infix::Infix(Expression *left, Token op, Expression *right) {
    type = ASTNodeType::Infix;

    this->left = left;
    this->op = op;
    this->right = right;
}
Expression* Infix::getLeft() {return left;}
Expression* Infix::getRight() {return right;}
Token Infix::getOp() {return op;}

Integer::Integer(Token token) {
    type = ASTNodeType::Integer;
    value = std::stoi(token.literal);
}
