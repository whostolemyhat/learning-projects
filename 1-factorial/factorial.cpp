// 1. Write a computer program which finds the factorial of a number entered by the user
// g++ -Wall name.cpp -o name
#include <iostream>
//using BigInt header file to use integers of arbitrary precision 
#include "BigInt.cpp"
using std::cout;
using std::cin;
using std::endl;

BigInt factorial(BigInt num) {
  if (num <= 1) {
    return 1;
  }

  return num * factorial(num - 1);
}

int main() {
  cout << "Enter a number: ";

  std::string user_input;
  cin >> user_input;

  if(!cin) { 
    cout << "That's not a valid input" << endl;
    return 1;
  }

  BigInt infConv = user_input;
  cout << infConv.toString() << endl;
  BigInt result = factorial(infConv);
  cout << result.toString() << endl;;
  return 0;
}

