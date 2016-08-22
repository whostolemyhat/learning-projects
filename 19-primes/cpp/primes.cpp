#include <vector>

std::vector<bool> createSieve(int limit) {
  std::vector<bool> primes = { true; limit };

  return primes;
}

int main() {
  int target = 32;

  std::vector<bool> sieve = createSieve(target);

  return 0;
}