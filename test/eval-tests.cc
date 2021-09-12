#include <gtest/gtest.h>
#include "../src/libraries.hh"

#include "../src/lexer.hh"
#include "../src/parser.hh"
#include "../src/evaluator.hh"


Object::Object* eval(std::wstring string) {
    Lexer lexer = Lexer(string);
    Parser parser = Parser(lexer);
    Program *program = parser.parse_program();

    return Evaluator().evaluate(program);
}

TEST(EvaluatorTests, IntegerEvaluation) {
    Object::Object* res = eval(L"52");

    ASSERT_NE(nullptr, res);

    ASSERT_EQ(Object::ObjectType::Integer, res->type);
    Object::Integer* integer = static_cast<Object::Integer*>(res);
    
    EXPECT_EQ((long long)52, integer->value);
    
    EXPECT_EQ(L"52", integer->str());
}


TEST(EvaluatorTests, BoolEvaluation) {
    std::vector<std::wstring> strings = {L"verdadero", L"falso"};
    std::vector<bool> values = {true, false};

    for (int i = 0; i < 2; i++) {
        Object::Object* res = eval(strings[i]);
        
        ASSERT_NE(nullptr, res);

        ASSERT_EQ(Object::ObjectType::Boolean, res->type);
        Object::Boolean* boolean = static_cast<Object::Boolean*>(res);

        EXPECT_EQ(values[i], boolean->value);
    
        EXPECT_EQ(strings[i], boolean->str());
    }
}


TEST(EvaluatorTests, AssignEvaluation) {
    std::vector<std::wstring> strings = {
        L"a:=falso \n a",
        L"h0la    := verdadero\n\th0la",
        L"número27 :=27\nnúmero27"
    };
    std::vector<std::wstring> values = {
        L"falso",
        L"verdadero",
        L"27"
    };

    for (int i = 0; i < 3; i++) {
        Object::Object* res = eval(strings[i]);
        
        ASSERT_NE(nullptr, res) << "i = " << i;
        EXPECT_EQ(values[i], res->str());
    }
}


TEST(EvaluatorTests, InfixEvaluation) {
    std::vector<std::wstring> strings = {
        L"3+3",
        L"2-5",
        L"(2+6)",
        L"(3+(8-1))",
        L"a := (35+7)-(2-10)+1. a" 
    };
    std::vector<std::wstring> values = {
        L"6",
        L"-3",
        L"8",
        L"10",
        L"51"
    };

    for (int i = 0; i < 1; i++) {
        Object::Object* res = eval(strings[i]);
        
        ASSERT_NE(nullptr, res) << "i = " << i;
        EXPECT_EQ(values[i], res->str());
    }
}
