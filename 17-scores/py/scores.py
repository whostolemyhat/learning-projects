import sys
import statistics

class Student:
    name = ''
    midterm = 0
    final = 0
    homework = None # list

    def __init__(self, name, midterm, final, homework):
        self.name = name
        self.midterm = midterm
        self.final = final
        # make sure homework is int
        self.homework = [int(i) for i in homework]

    def __repr__(self):
        return "{}: {}".format(self.name, self.grade())

    def grade(self):
        """Calculates grade of student"""
        if len(self.homework) == 0:
            print("No homework!")
        return 0.2 * float(self.midterm) + 0.4 * float(self.final) + 0.4 * float(statistics.median(self.homework))

def main():
    students = []

    print("Enter student's name, midterm, final and homework followed by Enter. Press Ctrl+Z to finish")

    lines = sys.stdin.readlines()
    for line in lines:
        args = line.split()

        # only create student if there's enough input (eg blank line)
        if len(args) >= 4:
            name, midterm, final, *homework = args
            student = Student(name, midterm, final, homework)
            students.append(student)

    # sorts in-place
    students.sort(key=lambda student: student.name)

    for student in students:
        print(student)

if __name__ == '__main__':
    main()
