#include <iostream>
#include <string>

using namespace std;

bool isPalindrome(string text) {
  int i = text.length() - 1;

  for(auto text : text) {
    if(i <= text.length() / 2) {
      cout << letter << " " << i  << " " << text[i] << letter == text[i] << endl;
    }

    i--;
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