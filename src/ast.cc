#include "ast.hh"
#include "libraries.hh"

Program::Program() {
    type = ASTNodeType::Program;
    expressions = std::vector<Expression* >();
}


Boolean::Boolean(bool value) {
    type = ASTNodeType::Boolean;
    this->value = value;
}


Integer::Integer(Token token) {
    type = ASTNodeType::Integer;
    value = std::stoi(token.literal);
}
