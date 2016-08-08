#include <iostream>
#include <vector>
#include <stdexcept>
#include <iomanip>

class Coin {
private:
  int amount;
  int value;

public:
  Coin(int amount = 0, int value = 0) : amount(amount), value(value) {}

  int total() {
    return amount * value;
  }
};

int get_input() {
  int input;
  std::cin >> input;

  if(!std::cin) {
    throw std::invalid_argument("That's no number!");
  }

  return input;
}


int get_total(std::vector<Coin> coins) {
  int total = 0;
  for(auto coin : coins) {
    total += coin.total();
  }

  return total;
}

int main() {
  std::vector<Coin> coins;
  std::cout << "Enter number of coins: " << std::endl;

  try {
    std::cout << "£2: ";
    coins.push_back(Coin(get_input(), 200));

    std::cout << "£1: ";
    coins.push_back(Coin(get_input(), 100));

    std::cout << "50p: ";
    coins.push_back(Coin(get_input(), 50));

    std::cout << "20p: ";
    coins.push_back(Coin(get_input(), 20));

    std::cout << "10p: ";
    coins.push_back(Coin(get_input(), 10));

    std::cout << "5p: ";
    coins.push_back(Coin(get_input(), 5));
  } catch(const std::invalid_argument& e) {
    std::cout << e.what() << std::endl;
  }

  // std::cout << std::setprecision(2);

  std::cout << "£" << get_total(coins) / 100.0 << std::endl;

  return 0;
}