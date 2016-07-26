#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> permutations(std::string word) {
  size_t len = word.length();

  if(len <= 1) {
    return std::vector<std::string> { word };
  }

  return std::vector<std::string> { word };

}

int main() {
  for(std::string result : permutations("TEST")) {
    std::cout << result;
  }

  return 0;
}