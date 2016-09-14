#include <iostream>
#include "util.h"
#include "html.h"

int main(int argc, char* argv[]) {
  if(argc == 1) {
    std::cerr << "Provide a file" << std::endl;
    return 1;
  }

  std::stringstream contents = open_file(argv[1]);

  std::cout << strip_tags(contents.str()) << std::endl;
}