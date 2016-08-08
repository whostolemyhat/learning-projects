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

int get_input() {
  int input;
  std::cin >> input;

  if(!std::cin) {
    std::cout << "That's no number!" << std::endl;
  }

  return input;
}

int main() {
  std::vector<Coin> coins;
  std::cout << "Enter number of coins: " << std::endl;

  std::cout << "£2: ";
  coins.push_back(Coin(get_input(), 200));

  for(auto coin : coins) {
    std::cout << coin.total() << std::endl;
  }

  return 0;
}