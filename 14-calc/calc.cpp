#include <iostream>
#include <string>

// float divide(int first, int second) {}
// int mulitply(int first, int second) {}
// int add(int first, int second) {}
// int subtract(int first, int second) {}

using std::string;

int main() {
  std::cout << "Enter a sum: ";

  string input;
  std::cin >> input;

  string operators[] = {"+", "-", "*", "/"};

  // search for operator in string
  for(auto op : operators) {
    std::cout << op << ": " << input.find(op) << std::endl;
  }

  return 0;
}