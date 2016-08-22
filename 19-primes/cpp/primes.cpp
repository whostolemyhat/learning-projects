#include <iostream>
#include <vector>

std::vector<bool> createSieve(int limit) {
  std::vector<bool> primes(limit, true);

  return primes;
}

int main() {
  int target = 32;

  std::vector<bool> sieve = createSieve(target);

  for(auto item : sieve) {
    std::cout << item << std::endl;
  }

  return 0;
}