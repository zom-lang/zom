/*
This file is to test the output object code.
*/

#include <iostream>

extern "C" {
    int foo();
}

int main() {
    std::cout << "foo" << foo(3.0, 4.0) << std::endl;
}