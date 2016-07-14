#include <iostream>
#include <string>

using std::cout;
using std::cin;
using std::endl;
using std::string;

int main() {
  cout << "Enter a number:";

  int num;
  cin >> num;

  if(!cin) {
    cout << "That's no number!";
    return 1;
  }

  while (num > 0) {
    cout << string(num, '*') << endl;
    num--;
  }

  return 0;
}