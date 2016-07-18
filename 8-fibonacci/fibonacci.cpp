#include <iostream>

int fib(i, total, target) {
  if(i == target) {
    return total;
  }

  i++;
  total = fib(i, total, target);
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