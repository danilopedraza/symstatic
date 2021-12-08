#include <iostream>

#include "lang/lexer.hh"
#include "lang/parser.hh"
#include "lang/evaluator.hh"

void repl() {
    std::wstring string;

    std::wcout << L">>> ";
    std::getline(std::wcin, string);
    while (string != L"salir") {
        

        Lexer lexer = Lexer(string);
        Parser parser = Parser(lexer);
        Program *program = parser.parse_program();


        std::wcout << Evaluator().evaluate_program(program)->str() << L'\n';

        std::wcout << L">>> ";
        std::getline(std::wcin, string);
    }
}


int main() {
    repl();
}
