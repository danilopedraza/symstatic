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
        ASTNode *ASTNode = parse_ASTNode(PRECEDENCES::LOWEST);
        if (ASTNode == nullptr) // there is an error
            break;
        else
            program->ASTNodes.push_back(ASTNode);
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
    advance_tokens(); // left brace
    
    Block *block = new Block();
    while (
        current_token.type != TokenType::RBRACE &&
        current_token.type != TokenType::EOFILE
    ) {
        ASTNode *exp = parse_ASTNode(PRECEDENCES::LOWEST);
        if (exp != nullptr)
            block->routine.push_back(exp);
        else
            return nullptr; // error (probably managed before arriving here)
    }
    
    if (!expected_token(TokenType::RBRACE))
        return nullptr; // error
    return block;
}


ASTNode* Parser::parse_ASTNode(PRECEDENCES precedence) {
    ASTNode *ASTNode = nullptr;
    
    switch (current_token.type) {
        case TokenType::INT:
            ASTNode = parse_integer();
            break;
        case TokenType::IDENT:
            if (peek_token.type == TokenType::ASSIGN)
                ASTNode = parse_assignment();
            else
                ASTNode = parse_identifier();
            break;
        case TokenType::TRUE:
            ASTNode = parse_boolean(true);
            break;
        case TokenType::FALSE:
            ASTNode = parse_boolean(false);
            break;
        case TokenType::LPAREN:
            ASTNode = parse_parenthesis();
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
        ASTNode = parse_infix(ASTNode);
    }

    expected_token(TokenType::POINT); // just pass if there is a point
    return ASTNode; 
}

ASTNode* Parser::parse_assignment() {
    std::wstring name = current_token.literal;
    advance_tokens(); // name token
    advance_tokens(); // assignment token
    
    ASTNode *value = parse_ASTNode(PRECEDENCES::LOWEST);
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

ASTNode* Parser::parse_infix(ASTNode *left) {
    Token op = current_token;
    PRECEDENCES cur_prec = current_precedence();
    advance_tokens(); // op token
    
    ASTNode *right = parse_ASTNode(cur_prec);
    
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
    ASTNode *exp = parse_ASTNode(PRECEDENCES::LOWEST);
    if (expected_token(TokenType::RPAREN)) // right parenthesis if there is one
        return exp;
    return nullptr; // error
}
