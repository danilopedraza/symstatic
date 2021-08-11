cmake_minimum_required(VERSION 3.16.3)

set(this symstatic)

project(${this} C CXX)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

enable_testing()

add_subdirectory(googletest)

set(headers
    src/ast.hh
    src/evaluator.hh
    src/lexer.hh
    src/libraries.hh
    src/object.hh
    src/parser.hh
    src/token.hh
)

set(sources
    src/ast.cc
    src/evaluator.cc
    src/lexer.cc
    src/object.cc
    src/parser.cc
    src/token.cc
)

add_library(${this} STATIC ${sources} ${headers})

add_subdirectory(test)
