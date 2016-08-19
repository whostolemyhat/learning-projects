// g++ -std=c++11 -Wall *.cpp -o output
#include "rot13.h"
#include <iostream>
#include <sstream>
#include <fstream>

int main(int argc, char* argv[]) {
  if(argc == 0) {
    std::cerr << "Provide a file to convert.\nUsage: (programme) filename.txt" << std::endl;
    return 1;
  }

  std::string filename = argv[1];

  std::ifstream text(filename);
  std::stringstream buffer;
  buffer << text.rdbuf();

  std::string converted = rot13(buffer.str());


  // std::cout << converted << std::endl;

  std::ofstream output;
  output.open(filename);
  output << converted;
  output.close();

  std::cout << "Converted " << filename << std::endl;

  return 0;
}