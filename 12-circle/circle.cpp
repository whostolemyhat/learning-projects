#include <iostream>

class Circle {
private:
  float area;
  float radius;
  float diameter;

public:
  Circle(float area = 0, float radius = 0, flaot diameter = 0) :
    area(area), radius(radius), diameter(diameter);

  void print() {
    std::cout << "Circle(area:" << area << ", radius: " << radius << ", diameter: " << diameter << std::endl;
  }
}

int main() {
  std::cout << "Enter size: ";
  int size;
  cin >> size;

  if(!cin) {
    std::cout << "Enter a number!" << std::endl;
    return 1;
  }

  Circle circle();
  circle.print();

  return 0;
}