#include <iostream>
#include <stdlib.h> // rand, srand
#include <time.h> // time
#include <stdlib.h> // abs

int main() {
  srand(time(NULL));

  int target = rand() % 100 + 1;
  bool correct = false;
  int guesses = 0;

  while(!correct) {
    std::cout << "Enter your guess: ";
    int guess;
    std::cin >> guess;
    guesses++;

    if(!std::cin) {
      std::cout << "That's not a number!" << std::endl;
      break;
    }

    int difference = abs(target - guess);
    if(difference < 10) {
      std::cout << "Close!" << std::endl;
    } else if(difference < 25) {
      std::cout << "Nearly." << std::endl;
    } else if(difference < 50) {
      std::cout << "Nope" << std:: endl;
    }

    if(guess < target) {
      std::cout << "Too low! Try again." << std::endl;
    } else if(guess > target) {
      std::cout << "Too high! Try again." << std::endl;
    } else {
      std::cout << "Correct! The magic number is " << target << std::endl;
      std::cout << "You found the magic number in " << guesses << " guesses" << std::endl;
      correct = true;
    }
  }

  return 0;
}