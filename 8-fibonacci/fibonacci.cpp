#include <iostream>

// this calcs nth fibonacci number
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


  // this prints out up to nth fibonacci
  int target = 5;
  int first = 1;
  int second = 1;
  // int temp;
  int output = 0;
  // for(int i = 0; i < target; i++) {
  while(output < target) {
    if(output == 0) {
      std::cout << first << " ";
      continue;
    }

    if(output == 1) {
      std::cout << second << " ";
      continue;
    }

    output = first + second;
    std::cout << output << " ";

    // change vars
    // temp = second;
    first = second;
    second = output;
  }

  return 0;
}