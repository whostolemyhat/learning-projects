#pragma once

#include <iostream>
#include <string>
#include <vector>

struct StudentInfo {
  std::string name;
  double final;
  double midterm;
  std::vector<double> homework;
};

bool compare(const StudentInfo&, const StudentInfo&);
std::istream& read(std::istream&, StudentInfo&);
std::istream& read_homework(std::istream&, std::vector<double>&);