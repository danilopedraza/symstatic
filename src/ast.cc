#include "ast.hh"
#include "libraries.hh"

Program::Program() {
    type = ASTNodeType::Program;
}


Assignment::Assignment(std::wstring name, ASTNode *value) {
    type = ASTNodeType::Assignment;
    this->name = name;
    this->value = value;
}
std::wstring Assignment::getName() {return name;}
ASTNode* Assignment::getValue() {return value;}
void Assignment::setValue(ASTNode *value) {this->value = value;}


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


If::If(ASTNode *condition, ASTNode *consequence, ASTNode *alternative) {
    type = ASTNodeType::If;
    this->condition = condition;
    this->consequence = consequence;
    this->alternative = alternative;
}
ASTNode *If::getCondition() {return condition;}
ASTNode *If::getConsequence() {return consequence;}
ASTNode *If::getAlternative() {return alternative;}


Infix::Infix(ASTNode *left, Token op, ASTNode *right) {
    type = ASTNodeType::Infix;

    this->left = left;
    this->op = op;
    this->right = right;
}
ASTNode* Infix::getLeft() {return left;}
ASTNode* Infix::getRight() {return right;}
Token Infix::getOp() {return op;}


Integer::Integer(Token token) {
    type = ASTNodeType::Integer;
    value = std::stoi(token.literal);
}


Minus::Minus(ASTNode *value) {
    type = ASTNodeType::Minus;
    this->value = value;
}
ASTNode *Minus::getValue() {return value;}


Not::Not(ASTNode *value) {
    type = ASTNodeType::Not;
    this->value = value;
}
ASTNode *Not::getValue() {return value;}

While::While(ASTNode *condition, ASTNode *routine) {
    type = ASTNodeType::While;
    this->condition = condition;
    this->routine = routine;
}
ASTNode* While::getCondition() {return condition;}
ASTNode* While::getRoutine() {return routine;}
