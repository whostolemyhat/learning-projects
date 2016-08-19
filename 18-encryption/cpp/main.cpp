// g++ -std=c++11 -Wall *.cpp -o output
#include "rot13.h"
#include <iostream>

int main(int argc, char* argv[]) {
  for (auto arg : argv) {
    std::cout << argv << std::endl;
  }

  std::cout << rot13("Hello World!") << std::endl;

  return 0;
}