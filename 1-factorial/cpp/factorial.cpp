// 1. Write a programme which finds the factorial of a number entered by the user
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

  if(!cin) {
    cout << "That's not a number." << endl;

    return 1;
  }

  cout << factorial(num) << endl;;

  return 0;
}