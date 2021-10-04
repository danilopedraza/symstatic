#include "libraries.hh"
#include "lexer.hh"
#include "token.hh"

Lexer::Lexer(std::wstring source) {
    this->source = source;
    position = read_position
             = line
             = 0;
    
    column = -1; //read_character() will put it in 0
    read_character();
}

Token Lexer::next_token() {
    skip_all_spaces();

    Token token;
    token.line = line;
    token.column = column;

    if      (is_number(current_character)) {
        token.type = TokenType::INT;
        token.literal = read_number();
    }
    else if (is_letter(current_character)) {
        token.literal = read_identifier();
        auto it = KEYWORDS.find(token.literal);

        if (it != KEYWORDS.end())
            token.type = it->second;
        else
            token.type = TokenType::IDENT;
    }
    else {
        switch (current_character) {
            case L':':
                if (peek_character() == L'=') {
                    token.literal = L":=";
                    token.type = TokenType::ASSIGN;
                    read_character();
                }
                break;
            case L'(':
                token.literal = L'(';
                token.type = TokenType::LPAREN;
                break;
            case L')':
                token.literal = L')';
                token.type = TokenType::RPAREN;
                break;
            case L'{':
                token.literal = L'{';
                token.type = TokenType::LBRACE;
                break;
            case L'}':
                token.literal = L'}';
                token.type = TokenType::RBRACE;
                break;
            case L'+':
                token.literal = L'+';
                token.type = TokenType::PLUS;
                break;
            case L'-':
                token.literal = L'-';
                token.type = TokenType::MINUS;
                break;
            case L'.':
                token.literal = L'.';
                token.type = TokenType::POINT;
                break;
            case L'*':
                token.literal = L'*';
                token.type = TokenType::MULTIPLICATION;
                break;
            case L'/':
                token.literal = L'/';
                token.type = TokenType::DIVISION;
                break;
            case L'=':
                token.literal = L'=';
                token.type = TokenType::EQUALS;
                break;
            case L';':
                token.literal = L';';
                token.type = TokenType::SEMICOLON;
                break;
            case WEOF:
                token.literal = WEOF;
                token.type = TokenType::EOFILE;
                break;
            default:
                token.type = TokenType::ILLEGAL;
                break;
        }
        read_character();
    }

    return token;
}

void Lexer::read_character() {
    if (read_position >= source.length())
        current_character = WEOF;
    else
        current_character = source[read_position];
    
    position = read_position;
    read_position++;

    column++;

    if (current_character == L'\n') {
        column = 0;
        line++;
    }
}

void Lexer::skip_all_spaces() {
    while (
        current_character == L' '  ||
        current_character == L'\n' ||
        current_character == L'\t'
    )
        read_character();
}

void Lexer::skip_lines() {
    while (peek_character() == L'\n') {
        read_character();
        line++;
    }
}

wchar_t Lexer::peek_character() {
    if (read_position >= source.length())
        return WEOF;
    
    return source[read_position];
}

bool Lexer::is_number(wchar_t chr) {
    return L'0' <= chr && chr <= L'9';
}

bool Lexer::is_letter(wchar_t chr) {
    if (
        (L'a' <= chr && chr <= L'z') ||
        (L'A' <= chr && chr <= L'Z')
    ) return true;

    std::wstring especials = L"_áäÁÄéëÉËíïÍÏóöÓÖúüÚÜñÑ";
    for (size_t i = 0; i < especials.size(); i++)
        if (especials[i] == chr) return true;
    
    return false;
}

std::wstring Lexer::read_number() {
    std::wstring num = L"";

    while (is_number(current_character)) {
        num += current_character;
        read_character();
    }
    
    return num;
}

std::wstring Lexer::read_identifier() {
    std::wstring id = L"";

    do {
        id += current_character;
        read_character();
    } while (is_number(current_character) || is_letter(current_character));
    
    return id;
}
