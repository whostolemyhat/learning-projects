class Matrix {
private:
  int _rows;
  int _cols;
  int _intial;

public:
  Matrix(int rows, int cols, int initial = 0)
  : _rows(rows), _cols(cols), _intial(intial) {}

  ~Matrix() {}
}