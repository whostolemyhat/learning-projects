// https://www.quantstart.com/articles/Matrix-Classes-in-C-The-Header-File
// http://code.activestate.com/recipes/578131-a-simple-matrix-class/
#pragma once
#include <vector>

template <typename T> class Matrix {
  private:
    std::vector<std::vector<T> > mat;
    unsigned rows;
    unsigned cols;

  public:
    Matrix(unsigned _rows, unsigned _cols, const T& _initial);
    Matrix(const Matrix<T>& rhs);
    virtual ~Matrix();

    Matrix<T>& operator=(const Matrix<T>& rhs);

    unsigned get_rows() const;
    unsigned get_cols() const;
}