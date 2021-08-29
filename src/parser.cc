#include "parser.hh"

#include "ast.hh"

Parser::Parser(Lexer lexer_) : lexer(lexer_) {
    ident_level = 0;
    advance_tokens();
    advance_tokens();
};

Program* Parser::parse_program() {
    Program* program = new Program();

    while (current_token.type != TokenType::EOFILE) {
        Expression *expression = parse_expression();
        if (expression == nullptr) // there is an error
            break;
        else
            program->expressions.push_back(expression);
    }

    return program;
}

void Parser::advance_tokens() {
    current_token = peek_token;
    peek_token = lexer.next_token();
}

bool Parser::expected_token(TokenType token_type) {
    if (peek_token.type == token_type) {
        advance_tokens();
        return true;
    }

    return false;
}


Expression* Parser::parse_expression() {
    Expression *expression = nullptr;
    
    switch (current_token.type) {
        case TokenType::INT:
            expression = parse_integer();
            break;
        case TokenType::IDENT:
            if (peek_token.type == TokenType::ASSIGN)
                expression = parse_assignment();
            else
                expression = parse_identifier();
            break;
        case TokenType::TRUE:
            expression = parse_boolean(true);
            break;
        case TokenType::FALSE:
            expression = parse_boolean(false);
            break;
        case TokenType::ILLEGAL:
            advance_tokens();
            break;
    }

    return expression; 
}

Expression* Parser::parse_assignment() {
    std::wstring name = current_token.literal;
    advance_tokens(); // name token
    advance_tokens(); // assignment token
    
    Expression *value = parse_expression();
    if (value != nullptr)
        return new Assignment(name, value);
    // else...
    return nullptr;
}

Expression* Parser::parse_boolean(bool value) {
    Boolean *boolean = new Boolean(value);
    advance_tokens();

    return boolean;
}

Expression* Parser::parse_identifier() {
    Identifier *ident = new Identifier(current_token.literal);
    advance_tokens();
    return ident;
}

Expression* Parser::parse_integer() {    
    Integer *integer = new Integer(current_token);
    advance_tokens();

    return integer;
}
