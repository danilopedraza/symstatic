cmake_minimum_required(VERSION 3.16.3)

set(this symstatic)

project(${this} C CXX)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

enable_testing()

add_subdirectory(googletest)

set(headers
    src/lexer.hh    
)

set(sources
    src/lexer.cc
)

add_library(${this} STATIC ${sources} ${headers})

add_subdirectory(test)
