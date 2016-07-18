#include <iostream>

// int fib(int x) {
//   // std::cout << x << std::endl;
//   if(x < 2) {
//     return 1;
//   }

//   return fib(x - 2) + fib(x - 1);
// }

int main() {
  // int num;
  // std::cin >> num;

  // if(!std::cin) {
  //   std::cout << "That's no number" << std::endl;
  // }

  // std::cout << fib(5);
  int target = 5;
  int first = 0;
  int second = 0;
  int temp;
  for(int i = 0; i < target; i++) {
    if(i == 0) {
      std::cout << first << std::endl;
      first = 1;
      break;
    }

    if(i == 1) {
      std::cout << second << std::endl;
      second = 1;
      break;
    }

    std::cout << first + second << std::endl;

    // change vars
    temp = second;
    first = second;
    second = first + temp;
  }

  return 0;
}