#include "ast.hh"
#include "libraries.hh"

Program::Program() {
    type = ASTNodeType::Program;
    expressions = std::vector<Expression* >();
}


Boolean::Boolean(Token token) {
    type = ASTNodeType::Boolean;
    this->value = (token.type == TokenType::TRUE);
}


Integer::Integer(Token token) {
    type = ASTNodeType::Integer;
    value = std::stoi(token.literal);
}
