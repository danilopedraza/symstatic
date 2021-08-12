#include "libraries.hh"
#include "lexer.hh"
#include "token.hh"

Lexer::Lexer(std::wstring source_) : source(source_) {
    position = read_position
             = line
             = 0;
    
    column = -1; //read_character() will put it in 0
    read_character();
};

Token Lexer::next_token() {
    skip_whitespace();

    Token token;
    token.line = line;
    token.column = column;

    switch (current_character) {
        case WEOF:
            token.literal = WEOF;
            token.type = TokenType::EOFILE;
            break;
        default:
            break;
    }

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
            token.type = TokenType::ILLEGAL;
    }
    else
        token.type = TokenType::ILLEGAL;

    read_character();

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

void Lexer::skip_whitespace() {
    while (current_character == L' ')
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
    std::wstring especials = L"áéíóúÁÉÍÓÚñÑ";
    
    for (size_t i = 0; i < especials.size(); i++)
        if (especials[i] == chr) return true;
    
    return (L'0' <= chr && chr <= L'9') || (L'a' <= chr && chr <= L'z') || (L'A' <= chr && chr <= L'Z');
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
    } while ((is_number(current_character) || is_letter(current_character))
          && position < source.length());
    
    return id;
}
