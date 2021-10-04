#include "ast.hh"
#include "lexer.hh"
#include "parser.hh"


Parser::Parser(Lexer lexer) : lexer(lexer) {
    advance_tokens();
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
    else if (current_token.type == TokenType::WHILE)
        node = parse_while();
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
    peek_token = peek_peek_token;
    peek_peek_token = lexer.next_token();
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
    bool is_a_block = expected_token(TokenType::LBRACE);
    
    Block *block = new Block();
    do {
        ASTNode *exp = parse_anything();
        if (exp != nullptr)
            block->routine.push_back(exp);
        else
            return nullptr; // error (probably managed before arriving here)
    } while (
        current_token.type != TokenType::RBRACE &&
        current_token.type != TokenType::EOFILE &&
        is_a_block
    );
    
    if (is_a_block && !expected_token(TokenType::RBRACE))
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
            node = parse_boolean();
            break;
        case TokenType::FALSE:
            node = parse_boolean();
            break;
        case TokenType::LPAREN:
            node = parse_parenthesis();
            break;
        case TokenType::MINUS:
            node = parse_minus();
            break;
        case TokenType::NOT:
            node = parse_not();
            break;
        case TokenType::FUNCTION:
            node = parse_function();
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
        if (current_token.type == TokenType::LPAREN)
            node = parse_call(node);
        else
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

ASTNode* Parser::parse_boolean() {
    Boolean *boolean = new Boolean(current_token.type == TokenType::TRUE);
    advance_tokens();

    return boolean;
}

ASTNode* Parser::parse_call(ASTNode *ident) {
    if (ident->type != ASTNodeType::Identifier)
        return nullptr; // error
    
    advance_tokens(); // left parenthesis token

    std::vector<ASTNode*> params;
    ASTNode *param = nullptr;
    while (
        current_token.type != TokenType::RPAREN &&
        current_token.type != TokenType::EOFILE
    ) {
        param = parse_expression(PRECEDENCES::LOWEST);
        if (param != nullptr)
            params.push_back(param);
        else
            return nullptr; // error
    }

    if (!expected_token(TokenType::RPAREN))
        return nullptr; // error

    return new FunctionCall(
        static_cast<Identifier*>(ident)->getName(),
        params
    );
}

ASTNode* Parser::parse_function() {
    advance_tokens(); // function token
    if (!expected_token(TokenType::LPAREN)) // left parenthesis token
        return nullptr;
    

    std::vector<ASTNode*> params;
    ASTNode *param = nullptr;
    while (
        current_token.type != TokenType::RPAREN &&
        current_token.type != TokenType::EOFILE
    ) {
        param = parse_expression(PRECEDENCES::LOWEST);
        if (param != nullptr && param->type == ASTNodeType::Identifier)
            params.push_back(param);
        else
            return nullptr; // error     
    }
        
    if (!expected_token(TokenType::RPAREN))
        return nullptr;
    
    Block* block = static_cast<Block*>(parse_block());
    if (block == nullptr)
        return nullptr; // error
    return new Function(params, block);
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
    else if (
        current_token.type   == TokenType::IF &&
        peek_token.type      == TokenType::NOT &&
        peek_peek_token.type == TokenType::THEN // si no entonces XD
    ) {
        advance_tokens();
        advance_tokens();
        advance_tokens();
        alternative = parse_block();
    }

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

ASTNode* Parser::parse_minus() {
    advance_tokens(); // minus token
    if (current_token.type == TokenType::MINUS)
        return nullptr; // error
    return new Minus(parse_expression(PRECEDENCES::LOWEST));
}

ASTNode* Parser::parse_not() {
    advance_tokens(); // not token
    if (current_token.type == TokenType::NOT)
        return nullptr; // error
    return new Not(parse_expression(PRECEDENCES::LOWEST));
}

ASTNode* Parser::parse_parenthesis() {
    advance_tokens(); // left parenthesis
    ASTNode *exp = parse_expression(PRECEDENCES::LOWEST);
    if (expected_token(TokenType::RPAREN)) // right parenthesis if there is one
        return exp;
    return nullptr; // error
}

ASTNode* Parser::parse_while() {
    advance_tokens(); // while token
    ASTNode *condition = parse_expression(PRECEDENCES::LOWEST);
    if (condition == nullptr)
        return nullptr; // error
    
    ASTNode *routine = parse_block();

    return new While(condition, routine);
}
