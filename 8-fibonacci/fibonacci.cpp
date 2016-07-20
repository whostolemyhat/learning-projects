#include <iostream>

// print out fibonacci sequence up til
int main() {
  std::cout << "Enter target number: ";
  int target;
  std::cin >> target;

  if(!std::cin) {
    std::cout << "That's no number" << std::endl;
  }

  int first = 0;
  int second = 1;
  int output = 0;
  int i = 0;

  while(output < target) {
    if(i <= 1) {
      output = i;
    } else {
      output = first + second;

      first = second;
      second = output;
    }

    if(output < target) {
      std::cout << output << " ";
    }

    i++;
  }

  return 0;
}