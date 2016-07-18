#include <iostream>

int fib(int x) {
  if(x <= 1) {
    return 1;
  }

  return fib(x - 1) + fib(x - 2);
}

int main() {
  int num;
  std::cin >> num;

  if(!std::cin) {
    std::cout << "That's no number" << std::endl;
  }

  std::cout << fib(5);

  return 0;
}