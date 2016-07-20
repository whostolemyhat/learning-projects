// Given a string, determine how many of the characters are vowels and
// how many are consonants. Terminate the string when the input
// character encountered is non-alphabetic.
#include <iostream>
#include <string>
#include <vector>
#include <algorithm> // std::find
#include <cctype> // isalpha

int main() {
  std::cout << "Enter a string:" << std::endl;

  std::string input;
  // input >> std::cin;
  getline(std::cin, input);

  if(!std::cin) {
    std::cout << "That's no string!" << std::endl;
  }

  std::cout << input << std::endl;

  // std::string vowels[] = { "a", "e", "i", "o", "u" };
  std::vector<char> vowels = {'a', 'e', 'i', 'o', 'u'};
  std::vector<char>::iterator it;
  int vowel_count = 0;
  int consonant_count = 0;

  for(auto c : input) {
    it = std::find(vowels.begin(), vowels.end(), c);

    // not in vector = returns index of end
    if(it != vowels.end()) {
      vowel_count++;
    } else {
      // ignore non-chars
      if (isalpha(c)) {
        consonant_count++;
      }
    }
  }

  std::cout << "Vowels: " << vowel_count << std::endl;
  std::cout << "Consonants: " << consonant_count << std::endl;

  return 0;
}