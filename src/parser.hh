#ifndef PARSER_HH
#define PARSER_HH

#include "libraries.hh"

#include "lexer.hh"
#include "ast.hh"

class Parser
{
public:
    Parser(Lexer lexer_);
    Program* parse_program();
private:
    Lexer lexer;
    Token current_token;
    Token peek_token;
    unsigned int ident_level;

    void advance_tokens();
    bool expected_token(TokenType token_type);

    std::unordered_set <TokenType> INFIX_OPERATORS = {
        TokenType::PLUS,
        TokenType::MINUS,
        TokenType::MULTIPLICATION,
        TokenType::DIVISION
    };

    enum class PRECEDENCES: int
    {
        LOWEST,
        SUM,
        PRODUCT
    };

    std::unordered_map <TokenType, PRECEDENCES> PRECEDENCE = {
        {TokenType::PLUS, PRECEDENCES::SUM},
        {TokenType::MINUS, PRECEDENCES::SUM},
        {TokenType::MULTIPLICATION, PRECEDENCES::PRODUCT},
        {TokenType::DIVISION, PRECEDENCES::PRODUCT}
    };

    PRECEDENCES current_precedence();
    
    Expression *parse_assignment();
    Expression* parse_expression(PRECEDENCES precedence);
    Expression* parse_boolean(bool value);
    Expression* parse_identifier();
    Expression* parse_infix(Expression *left);
    Expression* parse_integer();
    Expression* parse_parenthesis();
};

#endif
