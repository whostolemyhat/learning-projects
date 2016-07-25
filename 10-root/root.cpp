#include <iostream>
#include <cmath>

float fifthRoot(float x) {
  return pow(x, 1.0/5.0);
}

int main() {
  int sum = 0;

  for(int i = 0; i < 100; i++) {
    if(i % 2 == 0) {
      sum += i * i;
    }
  }

  std::cout << sum << fifthRoot((float) sum) << std::endl;

  return 0;
}