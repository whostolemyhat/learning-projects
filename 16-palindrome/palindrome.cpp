#include <iostream>
#include <string>
#include <cctype> // tolower
#include <regex>
#include <iterator> // back_inserter
#include <algorithm>

using namespace std;

bool isPalindrome(string text) {
  std::transform(text.begin(), text.end(), text.begin(), ::tolower);

  regex re("[^a-zA-Z0-9]");
  string stripped = std::regex_replace(text, re, "");

  int length = (int) stripped.length();
  int i = length - 1;

  for(auto letter : stripped) {
    if(i >= length / 2) {
      if(letter != stripped[i]) {
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

  // cin >> input only reads to first whitespace
  // so any spaces in phrase will break it
  getline(cin, input);

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