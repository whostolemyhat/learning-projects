// g++ -std=c++11 -Wall *.cpp -o output
#include "rot13.h"
#include <iostream>
#include <fstream>

int main(int argc, char* argv[]) {
  if(argc == 0) {
    std::cout << "Provide a file to convert." << std::endl;
    return 0;
  }

  for (int i = 0; i < argc; i++) {
    std::cout << argv[i] << std::endl;
  }

  std::string filename = argv[0];
  if(std::fstream is(filename)) {
    auto size = is.tellg(); // wtf
    std::string str(size, '\0');
    is.seekg(0);
    is.read(&str[0], size);
    std::cout << str << "\n";
  } else {
    std::cerr << "Couldn't open " << filename << std::endl;
  }

  std::cout << rot13("Hello World!") << std::endl;

  return 0;
}