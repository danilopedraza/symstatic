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
        L"a := falso \n a",
        L"hola := verdadero \n hola",
        L"número := 27 \n número"
    };
    std::vector<std::wstring> values = {
        L"falso",
        L"verdadero",
        L"27"
    };

    for (int i = 0; i < 3; i++) {
        Object::Object* res = eval(strings[i]);
        
        ASSERT_NE(nullptr, res);
        EXPECT_EQ(values[i], res->str());
    }
}
