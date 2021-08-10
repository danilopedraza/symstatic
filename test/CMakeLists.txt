cmake_minimum_required(VERSION 3.16.3)

set(this proj-tests)

set(sources
    eval-tests.cc
)

add_executable(${this} ${sources})
target_link_libraries(${this} PUBLIC
    gtest_main
    symstatic
)

add_test(
    NAME ${this}
    COMMAND ${this}
)
