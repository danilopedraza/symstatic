#ifndef PARSER_HH
#define PARSER_HH

#include "libraries.hh"

#include "lexer.hh"
#include "ast.hh"

class Parser
{
public:
    Parser(Lexer lexer);
    Program* parse_program();
private:
    Lexer lexer;
    Token current_token;
    Token peek_token;

    void advance_tokens();
    bool expected_token(TokenType token_type);

    std::unordered_set <TokenType> INFIX_OPERATORS = {
        TokenType::EQUALS,
        TokenType::PLUS,
        TokenType::MINUS,
        TokenType::MULTIPLICATION,
        TokenType::DIVISION
    };

    enum class PRECEDENCES: int
    {
        LOWEST,
        EQUALITY,
        SUM,
        PRODUCT
    };

    std::unordered_map <TokenType, PRECEDENCES> PRECEDENCE = {
        {TokenType::EQUALS, PRECEDENCES::EQUALITY},
        {TokenType::PLUS, PRECEDENCES::SUM},
        {TokenType::MINUS, PRECEDENCES::SUM},
        {TokenType::MULTIPLICATION, PRECEDENCES::PRODUCT},
        {TokenType::DIVISION, PRECEDENCES::PRODUCT}
    };

    PRECEDENCES current_precedence();
    
    ASTNode* parse_anything();
    ASTNode* parse_assignment();
    ASTNode* parse_block();
    ASTNode* parse_boolean(bool value);
    ASTNode* parse_expression(PRECEDENCES precedence);
    ASTNode* parse_identifier();
    ASTNode* parse_if();
    ASTNode* parse_infix(ASTNode *left);
    ASTNode* parse_integer();
    ASTNode* parse_minus();
    ASTNode* parse_parenthesis();
};

#endif
