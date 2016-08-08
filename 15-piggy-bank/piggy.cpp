#include <iostream>
#include <vector>

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

int main() {
  std::vector<Coin> coins;
  std::cout << "Enter number of coins: " << std::endl;

  std::cout << "£2: ";
  int two_pounds;
  std::cin >> two_pounds;

  if(!std::cin) {
    std::cout << "Enter a number!" << std::endl;
    return 1;
  }

  coins.push_back(Coin(two_pounds, 200));

  for(auto coin : coins) {
    std::cout << coin.total() << std::endl;
  }

  return 0;
}