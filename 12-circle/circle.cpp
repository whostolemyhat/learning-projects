#include <iostream>

class Circle {
private:
  float area;
  float radius;
  float diameter;

public:
  Circle(float area = 0, float radius = 0, float diameter = 0) :
    area(area), radius(radius), diameter(diameter) {}

  void print() {
    std::cout << "Circle(area:" << area << ", radius: " << radius << ", diameter: " << diameter << ")" << std::endl;
  }
};

int main() {
  std::cout << "Enter size: ";
  int size;
  std::cin >> size;

  if(!std::cin) {
    std::cout << "Enter a number!" << std::endl;
    return 1;
  }

  Circle circle(size);
  circle.print();

  return 0;
}