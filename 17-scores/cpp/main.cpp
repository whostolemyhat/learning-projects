// g++ *.cpp -o output
// This is example from Accelerated C++

#include <vector>
#include <string>
#include <algorithm>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include "StudentInfo.h"
#include "grade.h"

using namespace std;

int main() {
  vector<StudentInfo> students;
  StudentInfo record;
  string::size_type maxLength = 0;

  cout << "Hello! Please enter name, midterm, final and all homework grades.\n" <<
  "When you have finished, press Ctrl+D." << endl;

  while(read(cin, record)) {
    maxLength = max(maxLength, record.name.size());
    students.push_back(record);
  }

  sort(students.begin(), students.end(), compare);

  for(vector<StudentInfo>::size_type i = 0; i != students.size(); i++) {
    cout << students[i].name << string(maxLength + 1 - students[i].name.size(), ' ');

    try {
      double final_grade = grade(students[i]);
      streamsize prec = cout.precision();
      cout << setprecision(3) << final_grade << setprecision(prec);
    } catch (domain_error e) {
      cout << e.what();
    }

    cout << endl;
  }

  return 0;
}