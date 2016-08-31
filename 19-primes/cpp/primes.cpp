#include <iostream>
#include <vector>
#include <utility>
#include <cassert>

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

int main() {
  std::cout << "Enter a number: ";

  int target;

  std::cin >> target;

  if(!std::cin) {
    std::cout << "That's no number!" << std::endl;
    return 1;
  }

  std::vector<bool> sieve = createSieve(target);

  std::vector<std::pair<int, int>> results;
  for(int i = 0; i < target / 2; i++) {
    if(sieve[i] == true) {
      if(sieve[target - i] == true) {
        // std::pair<int, int> result(i, target - i);
        // emaplce_back adds straight into a pair in vec
        results.emplace_back(i, target - i);
      }
    }
  }

  // for(auto item : sieve) {
  //   std::cout << item << std::endl;
  // }

  for(auto result : results) {
    std::cout << result.first << ", " << result.second << std::endl;
  }

  // bit of testing :)
  assert(sieve[2] == true);
  assert(sieve[3] == true);
  assert(sieve[4] == false);
  assert(sieve[5] == true);
  assert(sieve[6] == false);
  assert(sieve[7] == true);
  assert(sieve[8] == false);
  assert(sieve[9] == false);
  assert(sieve[10] == false);
  assert(sieve[11] == true);
  assert(sieve[12] == false);
  assert(sieve[13] == true);
  assert(sieve[14] == false);
  assert(sieve[15] == false);
  assert(sieve[16] == false);
  assert(sieve[17] == true);
  assert(sieve[18] == false);
  assert(sieve[19] == true);
  assert(sieve[20] == false);
  assert(sieve[21] == false);
  assert(sieve[22] == false);
  assert(sieve[23] == true);
  assert(sieve[24] == false);
  assert(sieve[25] == false);
  assert(sieve[26] == false);
  assert(sieve[27] == false);
  assert(sieve[28] == false);
  assert(sieve[29] == true);
  assert(sieve[30] == false);
  assert(sieve[31] == true);
  assert(sieve[32] == false);

  return 0;
}