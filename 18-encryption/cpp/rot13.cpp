#include "rot13"
#include <unordered_map>

std::unordered_map<std::string, std::string> letters = {
  {"A", "N"},
  {"B", "O"},
  {"C", "P"},
  {"D", "Q"},
  {"E", "R"},
  {"F", "S"},
  {"G", "T"},
  {"H", "U"},
  {"I", "V"},
  {"J", "W"},
  {"K", "X"},
  {"L", "Y"},
  {"M", "Z"},
  {"a", "n"},
  {"b", "o"},
  {"c", "p"},
  {"d", "q"},
  {"e", "r"},
  {"f", "s"},
  {"g", "t"},
  {"h", "u"},
  {"i", "v"},
  {"j", "w"},
  {"k", "x"},
  {"l", "y"},
  {"m", "z"}
};

std::string switchChar(std::string letter) {
  return letters[letter];
}

std::string rot13(std::string message) {
  std::string converted;

  for(auto letter in message) {
    std::cout << letter << "  " << switchChar(letter) << std::endl;
  }
}