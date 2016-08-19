#include "rot13.h"
#include <unordered_map>
#include <cctype>

std::unordered_map<char, char> letters = {
  {'A', 'N'},
  {'B', 'O'},
  {'C', 'P'},
  {'D', 'Q'},
  {'E', 'R'},
  {'F', 'S'},
  {'G', 'T'},
  {'H', 'U'},
  {'I', 'V'},
  {'J', 'W'},
  {'K', 'X'},
  {'L', 'Y'},
  {'M', 'Z'},
  {'N', 'A'},
  {'O', 'B'},
  {'P', 'C'},
  {'Q', 'D'},
  {'R', 'E'},
  {'S', 'F'},
  {'T', 'G'},
  {'U', 'H'},
  {'V', 'I'},
  {'W', 'J'},
  {'X', 'K'},
  {'Y', 'L'},
  {'Z', 'M'},
  {'a', 'n'},
  {'b', 'o'},
  {'c', 'p'},
  {'d', 'q'},
  {'e', 'r'},
  {'f', 's'},
  {'g', 't'},
  {'h', 'u'},
  {'i', 'v'},
  {'j', 'w'},
  {'k', 'x'},
  {'l', 'y'},
  {'m', 'z'},
  {'n', 'a'},
  {'o', 'b'},
  {'p', 'c'},
  {'q', 'd'},
  {'r', 'e'},
  {'s', 'f'},
  {'t', 'g'},
  {'u', 'h'},
  {'v', 'i'},
  {'w', 'j'},
  {'x', 'k'},
  {'y', 'l'},
  {'z', 'm'}
};

char switchChar(char letter) {
  if(std::isalpha(letter)) {
    return letters[letter];
  } else {
    return letter;
  }
}

std::string rot13(std::string message) {
  std::string converted = "";

  for(auto letter : message) {
    converted += switchChar(letter);
  }

  return converted;
}