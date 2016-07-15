#include <iostream>
#include <string>
#include <stdlib.h>
#include <time.h>

std::string first() {
  return "I'm from the first function!";
}

std::string second() {
  return "Hello, I'm the second";
}

std::string third() {
  return "I'm tertiary";
}

int main() {
  srand(time(NULL));

  int num = rand() % 3 + 1;

  switch(num) {
    case 1:
      std::cout << first() << std::endl;
      break;

    case 2:
      std::cout << second() << std::endl;
      break;

    case 3:
      std::cout << third() << std::endl;
      break;

    default:
      std::cout << "You will definitely never see this" << std::endl;
      break;
  }

  return 0;
}