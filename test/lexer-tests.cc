#include <gtest/gtest.h>
#include "../src/libraries.hh"

#include "../src/lexer.hh"
#include "../src/token.hh"

TEST(LexerTests, ParenthesisTokenization) {
    std::vector<std::wstring> strings = {
        L"(25+7)-resultado",
        L"súperVariableLargaYLatina := acc + (5-a)"
    };
    std::vector<std::vector<TokenType>> values = {
        {
            TokenType::LPAREN, TokenType::INT, TokenType::PLUS, TokenType::INT,
            TokenType::RPAREN, TokenType::MINUS, TokenType::IDENT, TokenType::EOFILE
        },
        {
            TokenType::IDENT, TokenType::ASSIGN, TokenType::IDENT, TokenType::PLUS,
            TokenType::LPAREN, TokenType::INT, TokenType::MINUS,
            TokenType::IDENT, TokenType::RPAREN, TokenType::EOFILE
        }
    };

    for (int i = 0; i < 2; i++) {
        Lexer lexer = Lexer(strings[i]);

        for (int token = 0; token < (int)values[i].size(); token++) {
            EXPECT_EQ(values[i][token], lexer.next_token().type) << "i = " << i;
        }
    }
}
