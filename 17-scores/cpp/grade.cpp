#include <vector>
#include "grade.h"
#include "StudentInfo.h"
#include "median.h"

double grade(double midterm, double final, double homework) {
  return 0.2 * midterm +
    0.4 * final +
    0.4 * homework;
}

double grade(double midterm, double final, const std::vector<double>& homework) {
  if(homework.size() == 0) {
    throw std::domain_error("No homework!");
  }

  return grade(midterm, final, median(homework));
}

double grade(const StudentInfo& s) {
  return grade(s.midterm, s.final, s.homework);
}