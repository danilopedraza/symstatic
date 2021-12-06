#include <gtest/gtest.h>
#include "../src/lang/libraries.hh"

#include "../src/lang/lexer.hh"
#include "../src/lang/token.hh"

TEST(LexerTests, ParenthesisTokenization) {
    std::vector<std::wstring> strings = {
        L"(25+7)-resultado.",
        L"súperVariableLargaYLatina := acc + (5-a)"
    };
    std::vector<std::vector<TokenType>> values = {
        {
            TokenType::LPAREN, TokenType::INT, TokenType::PLUS, TokenType::INT,
            TokenType::RPAREN, TokenType::MINUS, TokenType::IDENT, TokenType::POINT, TokenType::EOFILE
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


TEST(LexerTests, KeyWordsTokenization) {
    std::vector<std::wstring> strings = {
        L"si verdadero \n{a:=5+1.}",
        L"para todo {hacer {a := (a o falso) y verdadero.} mientras seMantieneEstaCondición.}",

    };
    std::vector<std::vector<TokenType>> values = {
        {
            TokenType::IF, TokenType::TRUE, TokenType::LBRACE, TokenType::IDENT,
            TokenType::ASSIGN, TokenType::INT, TokenType::PLUS, TokenType::INT, TokenType::POINT
        },
        {
            TokenType::FOR, TokenType::IDENT, TokenType::LBRACE, TokenType::DO, TokenType::LBRACE,
            TokenType::IDENT, TokenType::ASSIGN, TokenType::LPAREN, TokenType::IDENT, TokenType::OR,
            TokenType::FALSE, TokenType::RPAREN, TokenType::AND, TokenType::TRUE, TokenType::POINT,
            TokenType::RBRACE, TokenType::WHILE, TokenType::IDENT, TokenType::POINT, TokenType::RBRACE
        }
    };

    for (int i = 0; i < 2; i++) {
        Lexer lexer = Lexer(strings[i]);

        for (int token = 0; token < (int)values[i].size(); token++) {
            Token cur = lexer.next_token();
            EXPECT_EQ(values[i][token], cur.type) << "i = " << i << " chr = " << cur.literal;
        }
    }
}
