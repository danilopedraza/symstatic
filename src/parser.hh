#ifndef PARSER_HH
#define PARSER_HH

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
 
    Expression* parse_expression();
    Expression* parse_boolean();
    Expression* parse_integer();
};

#endif
