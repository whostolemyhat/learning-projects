#include <iostream>

// this calcs nth fibonacci number
int fib(int x) {
  // std::cout << x << std::endl;
  if(x < 2) {
    return 1;
  }

  return fib(x - 2) + fib(x - 1);
}

int main() {
  // int num;
  // std::cin >> num;

  // if(!std::cin) {
  //   std::cout << "That's no number" << std::endl;
  // }


  // this prints out up to nth fibonacci
  int target = 5;
  int first = 0;
  int second = 1;

  int output = 0;
  int i = 0;
  // for(int i = 0; i < target; i++) {
  while(output < target) {
    if(output <= 1) {
      output = i;
    } else {
      output = first + second;

      first = second;
      second = output;
    }

    std::cout << output << " ";
    i++;
  }

  return 0;
}