#include <iostream>
#include <string>
#include <cctype> // tolower
#include <regex>
#include <iterator> // back_inserter

using namespace std;

bool isPalindrome(string text) {
  // to lowercase
  // remove non-alphanumeric
  text = tolower(text);
  string stripped;

  regex re = ("[^a-zA-Z0-9]");
  regex_replace(std::back_inserter(stripped), text.begin(), text.end(), re);

  cout << stripped;
  int length = (int) text.length();
  int i = length - 1;

  for(auto letter : text) {
    if(i >= length / 2) {
      cout << letter << " " << i  << " " << text[i] << (letter == text[i]) << endl;
      if(letter != text[i]) {
        return false;
      }

      i--;
    } else {
      break;
    }
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