#include <iostream>
#include <string>
#include <vector>

using std::string;

std::vector<string> permutations(string word) {
  size_t len = word.length();

  if(len <= 1) {
    return std::vector<string> { word };
  }

  string trimmed = word.substr(1, len);

  std::vector<string> perms = permutations(trimmed);
  string current_char = word.substr(0, 1);
  std::vector<string> result;

  for(string perm : perms) {
    for(int i = 0; i < (int)perms.size() + 1; i++) {
      std::cout << i << std::endl;

      string front = perm.substr(0, i - 1);
      string rest = perm.substr(i, perm.size());

      std::cout << front << " " << rest << std::endl;
      result.push_back(front + current_char + rest);
    }
  }

  return result;

}

int main() {
  for(string result : permutations("TEST")) {
    std::cout << result;
  }

  return 0;
}