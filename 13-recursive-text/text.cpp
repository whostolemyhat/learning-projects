#include <iostream>
#include <string>

string reverse(string output, string input, int n) {
  if(n == 0) {
    return output + input.at(0)
  }

  output += input.at(n);
  return reverse(output, input, n - 1);
}

int main() {
  std::cout << "Enter some text: ";

  string input;
  std::cin >> input;

  for(auto ch : input) {
    std::cout << ch << std::endl;
  }

  reverse("", input, input.len());

  return 0;
}