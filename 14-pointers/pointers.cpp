#include <iostream>

void swap(int& a, int& b) {
  a = a + b;
  b = a - b;
  a = a - b;
}

int main() {
  int a = 1;
  int b = 2;
  int c = 3;

  // a = a + b;
  // b = a - b;
  // a = a - b;

  swap(a, b);
  swap(b, c);

  std::cout << a << " " << b << std::endl;

  return 0;
}