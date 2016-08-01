#include <iostream>

int main() {
  std::cout << "Enter some text: ";

  string input;
  std::cin >> input;

  for(auto ch : input) {
    std::cout << ch << std::endl;
  }

  return 0;
}