cmake_minimum_required(VERSION 3.16.3)

set(this symstatic)

project(${this} C CXX)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

enable_testing()

add_subdirectory(googletest)

set(headers
    src/lang/ast.hh
    src/lang/evaluator.hh
    src/lang/lexer.hh
    src/lang/libraries.hh
    src/lang/object.hh
    src/lang/parser.hh
    src/lang/token.hh
)

set(sources
    src/lang/ast.cc
    src/lang/evaluator.cc
    src/lang/lexer.cc
    src/lang/object.cc
    src/lang/parser.cc
    src/lang/token.cc

    src/main.cc
)

add_library(${this} STATIC ${sources} ${headers})

add_subdirectory(test)
