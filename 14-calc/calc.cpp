float divide(int first, int second) {}
int mulitply(int first, int second) {}
int add(int first, int second) {}
int subtract(int first, int second) {}

using namespace std;

int main() {
  cout << "Enter a sum: ";

  string input;
  cin >> input;

  string[] operators = ["+", "-", "*", "/"];

  // search for operator in string
  for(auto op : operators) {
    cout op << ": " << s.find(op) << endl;
  }

  return 0;
}