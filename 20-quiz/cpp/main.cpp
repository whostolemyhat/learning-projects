#include <iostream>
#include <string>
#include <sstream>
#include <fstream>

// https://github.com/nlohmann/json
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
  if(argc == 1) {
    std::cerr << "Provide a question file" << std::endl;
    return 1;
  }

  std::string filename = argv[1];

  std::stringstream contents = open_file(filename);

  // convert from JSON
  json data = json::parse(contents.str());

  std::cout << data[0] << std::endl;
  // read questions

  return 0;
}