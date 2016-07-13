// read user input
// check it's a number
// find factorial
// 4! = 4 + 3 + 2 + 1 = 10
#include <iostream>

using std::cout;
using std::cin;
using std::endl;

int factorial(int num) {
  if (num < 1) {
    return 0;
  }

  return num + factorial(num - 1);
}

int main() {
  cout << "Enter a number: ";

  int num;
  cin >> num;

  cout << factorial(num);

  return 0;
}