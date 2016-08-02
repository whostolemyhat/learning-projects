#include <iostream>

int main() {
  int a = 1;
  int b = 2;
  int c = 3;

  a = a + b;
  b = a - b;
  a = a - b;

  std::cout << a << " " << b << std::endl;

  return 0;
}