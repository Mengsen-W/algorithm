/*
 * @Date: 2021-03-21 09:36:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-21 09:59:27
 */

#include <cassert>
#include <vector>
using namespace std;

void set_zeroes(vector<vector<int>>& matrix) {
  int n = matrix.size(), m = matrix[0].size();
  bool row_0 = false, col_0 = false;  //标记第一行和第一列是否存在零元素

  for (int j = 0; j < m; j++)  //判断第一行是否有0
    if (matrix[0][j] == 0) {
      row_0 = true;
      break;
    }

  for (int i = 0; i < n; i++)  //判断第一列是否有0
    if (matrix[i][0] == 0) {
      col_0 = true;
      break;
    }

  for (int i = 1; i < n; i++)  //遍历数组内部的0
    for (int j = 1; j < m; j++)
      if (matrix[i][j] == 0)                 //若该元素为0
        matrix[i][0] = 0, matrix[0][j] = 0;  //则将此0元素的行首和列首元素置0

  for (int i = 1; i < n; i++)
    for (int j = 1; j < m; j++)
      if (matrix[i][0] == 0 ||
          matrix[0][j] == 0)  //该元素的行首或列首为0，则置0
        matrix[i][j] = 0;

  if (row_0)
    for (int i = 0; i < m; i++) matrix[0][i] = 0;  //第一行置0

  if (col_0)
    for (int i = 0; i < n; i++) matrix[i][0] = 0;  //第一列置0
}

int main() {
  vector<vector<int>> input = {{1, 1, 1}, {1, 0, 1}, {1, 1, 1}};
  set_zeroes(input);
  assert(input == move(vector<vector<int>>{{1, 0, 1}, {0, 0, 0}, {1, 0, 1}}));
  input = {{0, 1, 2, 0}, {3, 4, 5, 2}, {1, 3, 1, 5}};
  set_zeroes(input);
  assert(input ==
         move(vector<vector<int>>{{0, 0, 0, 0}, {0, 4, 5, 0}, {0, 3, 1, 0}}));
}