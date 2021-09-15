#include "ast.hh"
#include "lexer.hh"
#include "parser.hh"


Parser::Parser(Lexer lexer) : lexer(lexer) {
    advance_tokens();
    advance_tokens();
};

ASTNode* Parser::parse_anything() {
    ASTNode *node = nullptr;
    if (
        current_token.type == TokenType::IDENT &&
        peek_token.type    == TokenType::ASSIGN
    )
        node = parse_assignment();
    
    else if (current_token.type == TokenType::IF)
        node = parse_if();
    else
        node = parse_expression(PRECEDENCES::LOWEST);
    
    return node;
}

Program* Parser::parse_program() {
    Program* program = new Program();

    while (current_token.type != TokenType::EOFILE) {
        ASTNode *node = parse_anything();
        
        if (node == nullptr)
            break; // error
        else
            program->ASTNodes.push_back(node);
        
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


ASTNode* Parser::parse_block() {
    if (!expected_token(TokenType::LBRACE))
        return nullptr; //error
    
    Block *block = new Block();
    while (
        current_token.type != TokenType::RBRACE &&
        current_token.type != TokenType::EOFILE
    ) {
        ASTNode *exp = parse_anything();
        if (exp != nullptr)
            block->routine.push_back(exp);
        else
            return nullptr; // error (probably managed before arriving here)
    }
    
    if (!expected_token(TokenType::RBRACE))
        return nullptr; // error
    return block;
}


ASTNode* Parser::parse_expression(PRECEDENCES precedence) {
    ASTNode *node = nullptr;
    
    switch (current_token.type) {
        case TokenType::INT:
            node = parse_integer();
            break;
        case TokenType::IDENT:
            node = parse_identifier();
            break;
        case TokenType::TRUE:
            node = parse_boolean(true);
            break;
        case TokenType::FALSE:
            node = parse_boolean(false);
            break;
        case TokenType::LPAREN:
            node = parse_parenthesis();
            break;
        case TokenType::MINUS:
            advance_tokens();
            if (current_token.type == TokenType::MINUS)
                return nullptr; // error
            node = new Minus(parse_expression(PRECEDENCES::LOWEST));
            break;
        case TokenType::POINT:
            advance_tokens(); // error
            break;
        default: // error
            return nullptr;
    }

    while (
        INFIX_OPERATORS.count(current_token.type) &&
        precedence < current_precedence()
    ) {
        node = parse_infix(node);
    }

    expected_token(TokenType::POINT); // just pass if there is a point
    return node; 
}

ASTNode* Parser::parse_assignment() {
    std::wstring name = current_token.literal;
    advance_tokens(); // name token
    advance_tokens(); // assignment token
    
    ASTNode *value = parse_expression(PRECEDENCES::LOWEST);
    if (value != nullptr)
        return new Assignment(name, value);
    // else...
    return nullptr;
}

ASTNode* Parser::parse_boolean(bool value) {
    Boolean *boolean = new Boolean(value);
    advance_tokens();

    return boolean;
}

ASTNode* Parser::parse_identifier() {
    Identifier *ident = new Identifier(current_token.literal);
    advance_tokens();
    return ident;
}


ASTNode* Parser::parse_if() {
    advance_tokens(); // if token
    ASTNode *condition = parse_expression(PRECEDENCES::LOWEST);
    if (condition == nullptr)
        return nullptr; // error
    
    ASTNode *consequence = parse_block();

    ASTNode *alternative = nullptr;
    if (expected_token(TokenType::ELSE))
        alternative = parse_block();

    return new If(condition, consequence, alternative);
}


ASTNode* Parser::parse_infix(ASTNode *left) {
    Token op = current_token;
    PRECEDENCES cur_prec = current_precedence();
    advance_tokens(); // op token
    
    ASTNode *right = parse_expression(cur_prec);
    
    if (right == nullptr)
        return nullptr;

    return new Infix(left, op, right);
}

ASTNode* Parser::parse_integer() {
    Integer *integer = new Integer(current_token);
    advance_tokens();

    return integer;
}

ASTNode* Parser::parse_parenthesis() {
    advance_tokens(); // left parenthesis
    ASTNode *exp = parse_expression(PRECEDENCES::LOWEST);
    if (expected_token(TokenType::RPAREN)) // right parenthesis if there is one
        return exp;
    return nullptr; // error
}
