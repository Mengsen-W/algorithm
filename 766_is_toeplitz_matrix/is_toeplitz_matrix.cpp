/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-27 17:55:16
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-27 18:04:35
 */

#include <cassert>
#include <vector>

using namespace std;

bool is_toeplitz_matrix(vector<vector<int>>& matrix) {
  int row_size = matrix.size(), col_size = matrix[0].size();
  for (int i = 1; i < row_size; i++) {
    for (int j = 1; j < col_size; j++) {
      if (matrix[i][j] != matrix[i - 1][j - 1]) {
        return false;
      }
    }
  }
  return true;
}

int main(void) {
  vector<vector<int>> matrix;
  matrix = {{1, 2, 3, 4}, {5, 1, 2, 3}, {9, 5, 1, 2}};
  assert(is_toeplitz_matrix(matrix));
  matrix = {{1, 2}, {2, 2}};
  assert(!is_toeplitz_matrix(matrix));
  return 0;
}