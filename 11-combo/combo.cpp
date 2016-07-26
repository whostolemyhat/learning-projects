#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> permutations(std::string word) {
  size_t len = word.length();

  if(length <= 1) {
    return std::vector<std::string> { word };
  }

  return std::vector<std::string> { word };

}

int main() {
  std::cout << permutations("TEST") << std::endl;

  return 0;
}