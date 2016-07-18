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
  int first = 1;
  int second = 1;
  int temp;
  for(int i = 0; i < target; i++) {
    if(target < 2) {
      std::cout << first << std::endl;
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