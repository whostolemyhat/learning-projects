#include <iostream>
#include <math.h> // M_PI, sqrt

class Circle {
private:
  float area;
  float radius;
  float diameter;

public:
  Circle(float area = 0, float radius = 0, float diameter = 0) :
    area{area}, radius{radius}, diameter{diameter} {}

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
  std::cout << "Pick an option: " << std::endl;
  std::cout << "1. Area" << std::endl;
  std::cout << "2. Radius" << std::endl;
  std::cout << "3. Diameter" << std::endl;

  int choice;
  std::cin >> choice;

  if(!std::cin) {
    std::cout << "Enter a number!" << std::endl;
    return 1;
  }

  std::cout << "Enter size: ";
  int size;
  std::cin >> size;

  if(!std::cin) {
    std::cout << "Enter a number!" << std::endl;
    return 1;
  }

  Circle circle;
  switch(choice) {
    case 1:
      circle = Circle::from_area(size);
      break;
    case 2:
      circle = Circle::from_radius(size);
      break;
    case 3:
      circle = Circle::from_diameter(size);
      break;
    default:
      std::cout << "Pick an option from the list!" << std::endl;
      return 1;
  }

  circle.print();

  return 0;
}