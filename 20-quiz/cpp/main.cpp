#include <iostream>
#include <string>
#include "json.h"

std::stringstream open_file(std::string filename) {
  std::ifstream text(filename);
  std::stringstream buffer;
  buffer << text.rdbuf();

  return buffer;
}

int main(int argc, char* argv[]) {
  // get filename from args
  if(argc == 0) {
    std::cerr << "Provide a question file" << std::endl;
  }

  std::string filename = argv[1];

  // read file
  std::stringstream contents = open_file(filename);

  // convert from JSON
  Json::Value data;
  data = contents;

  std::cout << data;
  // read questions
  return 0;
}