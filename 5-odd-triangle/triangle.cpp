#include <iostream>
#include <string>

int main() {
  std::cout << "Enter a number: ";

  int num;
  std::cin > num;

  if(!std::cin) {
    std::cout << "That's no number!";
    return 1;
  }

  for(int i = 1; i < num + 1; i += 2) {
    std::cout << std::string(i, '*') << std::endl;
  }

  return 0;
}