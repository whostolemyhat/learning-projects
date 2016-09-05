#include <iostream>
#include <string>
#include "json.h"

using json = nlohmann::json;

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
  std::ifstream text(filename);
  std::stringstream buffer;
  buffer << text.rdbuf();
  // std::stringstream contents = open_file(filename);

  // convert from JSON
  json data = json::parse(contents.str());

  std::cout << data;
  // read questions
  return 0;
}