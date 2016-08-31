#include <iostream>
#include <vector>

std::vector<bool> createSieve(int limit) {
  std::vector<bool> primes(limit + 1, true);

  primes[0] = false;
  primes[1] = false;

  int i = 0;
  for(auto item : primes) {
    if(item == true) {
      int step = i;
      int n = i * i;

      while(n < limit) {
        primes[n] = false;
        n += step;
      }
    }

    i++;
  }

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