#include <iostream>
#include <math.h> // M_PI, sqrt

class Circle {
private:
  float area;
  float radius;
  float diameter;

public:
  Circle(float area = 0, float radius = 0, float diameter = 0) :
    area(area), radius(radius), diameter(diameter) {}

  static Circle from_radius(float radius) {
    return Circle(M_PI * (radius * radius), radius, radius * 2);
  }

  static Circle from_diameter(float diameter) {
    float radius = diameter / 2.0;

    return Circle(M_PI * (radius * radius), radius, diameter);
  }

  static Circle from_area(float area) {
    float diameter = sqrt(area / M_PI);

    return Circle(area, diameter / 2.0, diameter);
  }

  void print() {
    std::cout << "Circle(area: " << area
      << ", radius: " << radius
      << ", diameter: " << diameter << ")" << std::endl;
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

  Circle c2 = Circle::from_radius(12);
  c2.print();

  Circle c3 = Circle::from_diameter(12);
  c3.print();

  Circle c4 = Circle::from_area(12);
  c4.print();

  return 0;
}