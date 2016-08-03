#include <iostream>
#include <string>
#include <sstream>

// float divide(int first, int second) {}
// int mulitply(int first, int second) {}
// int add(int first, int second) {}
// int subtract(int first, int second) {}

// using std::string;
using namespace std;

int main() {
  std::cout << "Enter a sum: ";

  string input;
  getline(cin, input);

  string operators[] = {"+", "-", "*", "/"};

  // search for operator in string
  for(auto op : operators) {
    std::cout << op << ": " << input.find(op) << std::endl;
  }

  return 0;
}