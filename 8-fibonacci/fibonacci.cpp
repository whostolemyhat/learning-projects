#include <iostream>

int fib(int x) {
  std::cout << x << std::endl;
  if(x < 2) {
    return 1;
  }

  return fib(x - 2) + fib(x - 1);
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