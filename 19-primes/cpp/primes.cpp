#include <iostream>
#include <vector>

std::vector<bool> createSieve(int limit) {
  std::vector<bool> primes = { limit, true };

  return primes;
}

int main() {
  int target = 32;

  std::vector<bool> sieve = createSieve(target);

  std::cout << sieve << std::endl;

  return 0;
}