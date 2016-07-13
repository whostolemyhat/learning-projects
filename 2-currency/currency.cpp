// Develop a programme to convert currency X to currency Y and vice versa
// £1 = £1.32 lol
#include <iostream>
#include <iomanip>
#include <string>

using std::cout;
using std::cin;
using std::endl;
using std::string;

const float POUNDS_TO_DOLLARS = 1.32;
const float DOLLARS_TO_POUNDS = 0.75;

float convert(float amount, string type) {
  if(type == "pounds") {
    return amount * POUNDS_TO_DOLLARS;
  }

  return amount * DOLLARS_TO_POUNDS;
}

int main() {
  // todo: do while
  cout << "Choose from the following options (type number to begin):" << endl;
  cout << "(1) Pounds to dollars" << endl;
  cout << "(2) Dollars to pounds" << endl;

  int choice;
  cin >> choice;

  if(!cin) {
    cout << "Please type the number of your choice";
    return 1;
  }

  string type;
  string sign;
  string convertedSign;
  float amount;

  switch(choice) {
    case 1:
      cout << "Enter amount in £";
      type = "pounds";
      sign = "£";
      convertedSign = "$";
      break;

    case 2:
      cout << "Enter amount in $";
      type = "dollars";
      sign = "$";
      convertedSign = "£";
      break;

    default:
      cout << "Please pick an option from the list";
      return 1;
  }

  cin >> amount;
  if(!cin) {
    cout << "Enter a number" << endl;

    return 1;
  }

  cout << sign << amount << " is "
    << convertedSign << std::fixed << std::setprecision(2)
    << convert(amount, type) << endl;

  return 0;
}