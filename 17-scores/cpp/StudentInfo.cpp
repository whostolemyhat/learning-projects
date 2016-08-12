#include "StudentInfo.h"

#include <iostream>

using std::istream;
using std::vector;

bool compare(const StudentInfo& x, const StudentInfo& y) {
  return x.name < y.name;
}

istream& read(istream& in, StudentInfo& student) {
  in >> student.name >> student.midterm >> student.final;
  read_homework(in, student.homework);

  std::cout << "StudentInfo created" << std::endl;

  return in;
}

istream& read_homework(istream& in, vector<double>& homework) {
  if(in) {
    homework.clear();

    double x;
    while(in >> x) {
      homework.push_back(x);
    }

    in.clear();
  }

  return in;
}