#include <iostream>
#include <string>
#include <sstream> // getline
#include <map>
#include <tuple>

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

// using std::string;
using namespace std;

int main() {
  // auto addTuple = std::make_tuple(&Calc::add);
  // map <string, tuple>
  std::map<string, float(Calc::*)(int, int)> map;
  map["+"] = &Calc::add;
  map["-"] = &Calc::subtract;
  map["*"] = &Calc::mulitply;
  map["/"] = &Calc::divide;

  std::cout << "Enter a sum: ";

  string input;
  getline(cin, input);

  string operators[] = {"+", "-", "*", "/"};

  // search for operator in string
  // if not in string, returns number greater than string length
  // can also use std::string::npos
  for(auto op : operators) {
    std::size_t found = input.find(op);

    if(found != std::string::npos) {
      // std::cout << op << ": " << input.find(op) << std::endl;
      // split string at operator
      // get first and second numbers
      auto start = 0;
      auto end = found;

      // stoi
      int x = std::stoi(input.substr(start, end - start));
      int y = std::stoi(input.substr(end + op.length(), input.length()));

      std::cout << x << std::endl;
      std::cout << y << std::endl;

      Calc calc;
      auto func = map.find(op)->second;

      // TODO: check func exists

      // can't use strings in switch
      // get string:function pair out of map
      // call function
      // std::cout << map.find(op)->first << " = " << (calc.*(func->second))(x, y) << std::endl;
    }
  }

  return 0;
}