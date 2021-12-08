#include <iostream>

#include "lang/lexer.hh"
#include "lang/parser.hh"
#include "lang/evaluator.hh"

void repl() {
    std::wstring lines, line;

    std::wcout << L">>> ";
    std::getline(std::wcin, line);
    while (line != L"salir") {
        lines += line;

        Lexer lexer = Lexer(lines);
        Parser parser = Parser(lexer);
        Program *program = parser.parse_program();


        std::wcout << Evaluator().evaluate_program(program)->str() << L'\n';

        std::wcout << L">>> ";
        std::getline(std::wcin, line);
    }
}


int main() {
    repl();
}
