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
        Expression *expression = parse_expression(PRECEDENCES::LOWEST);
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
    if (current_token.type == token_type) {
        advance_tokens();
        return true;
    }

    return false;
}

Parser::PRECEDENCES Parser::current_precedence() {
    if (PRECEDENCE.count(current_token.type))
        return PRECEDENCE[current_token.type];
    return PRECEDENCES::LOWEST;
}


Expression* Parser::parse_expression(PRECEDENCES precedence) {
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
        case TokenType::LPAREN:
            expression = parse_parenthesis();
            break;
        case TokenType::POINT:
            advance_tokens();
            break;
        default: // error
            return nullptr;
    }

    while (
        INFIX_OPERATORS.count(current_token.type) &&
        peek_token.type != TokenType::POINT &&
        precedence < current_precedence()
    ) {
        expression = parse_infix(expression);
    }

    expected_token(TokenType::POINT); // just pass if there is a point
    return expression; 
}

Expression* Parser::parse_assignment() {
    std::wstring name = current_token.literal;
    advance_tokens(); // name token
    advance_tokens(); // assignment token
    
    Expression *value = parse_expression(PRECEDENCES::LOWEST);
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

Expression* Parser::parse_infix(Expression *left) {
    // TODO XD
    Token op = current_token;
    PRECEDENCES cur_prec = current_precedence();
    advance_tokens(); // op token
    
    Expression *right = parse_expression(cur_prec);
    
    if (right == nullptr)
        return nullptr;

    return new Infix(left, op, right);
}

Expression* Parser::parse_integer() {
    // TODO: pass number to Integer(), not the entire token
    Integer *integer = new Integer(current_token);
    advance_tokens();

    return integer;
}

Expression* Parser::parse_parenthesis() {
    advance_tokens(); // left parenthesis
    Expression *exp = parse_expression(PRECEDENCES::LOWEST);
    if (expected_token(TokenType::RPAREN)) // right parenthesis if there is one
        return exp;
    return nullptr; // error
}
