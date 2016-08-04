#include <iostream>
#include <string>
#include <sstream> // getline
#include <map>
#include <tuple>

// make sure all methods have same signatures so you can add to map
struct Calc {
  float divide(int first, int second) {
    return (float)first / second;
  }

  float mulitply(int first, int second) {
    return (float)first * second;
  }

  float add(int first, int second) {
    return (float)first + second;
  }

  float subtract(int first, int second) {
    return (float)first - second;
  }
};

using namespace std;

int main() {
  // map operator to function pointer
  std::map<string, float(Calc::*)(int, int)> map;
  map["+"] = &Calc::add;
  map["-"] = &Calc::subtract;
  map["*"] = &Calc::mulitply;
  map["/"] = &Calc::divide;

  std::cout << "Enter a sum, or 'q' to quit" << std::endl;

  while (true) {
    string input;
    getline(cin, input);

    string operators[] = {"+", "-", "*", "/"};

    if(input == "q") {
      break;
    }

    // search for operator in string
    // if not in string, returns number greater than string length
    // can also use std::string::npos
    for(auto op : operators) {
      std::size_t found = input.find(op);

      if(found != std::string::npos) {
        // split string at operator
        // get first and second numbers
        auto start = 0;
        auto end = found;

        int x = std::stoi(input.substr(start, end - start));
        int y = std::stoi(input.substr(end + op.length(), input.length()));

        Calc calc;
        auto func = map.find(op)->second;

        // TODO: check func exists

        // can't use strings in switch
        // get string:function pair out of map
        // call function
        std::cout << (calc.*(func))(x, y) << std::endl;
      }
    }
  }

  return 0;
}