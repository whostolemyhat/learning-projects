// Develop a programme to convert currency X to currency Y and vice versa
// £1 = £1.32 lol
#include <iostream>

using std::cout;
using std::cin;
using std::endl;

const float DOLLAR_CONVERSION = 1.32;

int main() {
  cout << "Enter amount in £:";

  float pounds;
  cin >> pounds;

  if(!cin) {
    cout << "Enter a number" << endl;
  }

  cout << "£" << pounds << " is $" << pounds * DOLLAR_CONVERSION << endl;

  return 0;
}