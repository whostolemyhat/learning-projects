#include "sieve.h"

std::vector<bool> createSieve(int limit) {
  std::vector<bool> primes(limit + 1, true);

  primes[0] = false;
  primes[1] = false;

  int i = 0;
  for(auto item : primes) {
    if(item == true) {
      int step = i;
      int n = i * i;

      while(n <= limit ) {
        primes[n] = false;
        n += step;
      }
    }

    i++;
  }

  return primes;
}