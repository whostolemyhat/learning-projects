// Given a string, determine how many of the characters are vowels and
// how many are consonants. Terminate the string when the input
// character encountered is non-alphabetic.
#include <iostream>
#include <string>

int main() {
  std::cout << "Enter a string:" << std::endl;

  std::string input >> cin;
  if(!cin) {
    std::cout << "That's no string!" << std::endl;
  }


  return 0;
}