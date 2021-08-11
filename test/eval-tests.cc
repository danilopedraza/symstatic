#include <gtest/gtest.h>

#include "../src/lexer.hh"
#include "../src/parser.hh"
#include "../src/evaluator.hh"


TEST(EvaluatorTests, IntegerEvaluation) {
    Lexer lexer = Lexer(L"5");
    Parser parser = Parser(lexer);
    Program *program = parser.parse_program();
    Evaluator eval = Evaluator();
    Object::Object* res = eval.evaluate(program);

    ASSERT_NE(nullptr, res);

    ASSERT_EQ(Object::ObjectType::Integer, res->type);
    Object::Integer* integer = static_cast<Object::Integer*>(res);
    
    EXPECT_EQ((long long)5, integer->value);
    
    EXPECT_EQ(L"5", integer->str());
}
