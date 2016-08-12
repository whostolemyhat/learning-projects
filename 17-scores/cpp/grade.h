#pragma once

#include <vector>
#include "StudentInfo.h"

double grade(double, double, double);
double grade(double, double, const std::vector<double>&);
double grade(const StudentInfo&);