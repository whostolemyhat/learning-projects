#include <iostream>
#include <string>

using namespace std;

bool isPalindrome(string text) {
  int i = 0;
  for(auto letter : text) {
    cout << letter << " " << i << endl;
  }

  return true;
}

int main() {
  cout << "Enter some text: ";
  string input;
  cin >> input;

  if(!cin) {
    cout << "Failed to read input" << endl;
  }

  if(isPalindrome(input)) {
    cout << "Yep!" << endl;
  } else {
    cout << "Nope" << endl;
  }

  return 0;
}