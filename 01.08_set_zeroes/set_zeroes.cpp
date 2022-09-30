/*
 * @Date: 2022-09-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-30
 * @FilePath: /algorithm/01.08_set_zeroes/set_zeroes.cpp
 */

#include <assert.h>

#include <vector>

using namespace std;

class Solution {
 public:
  void setZeroes(vector<vector<int>>& matrix) {
    int m = matrix.size();
    int n = matrix[0].size();
    int flag_col0 = false;
    for (int i = 0; i < m; i++) {
      if (!matrix[i][0]) {
        flag_col0 = true;
      }
      for (int j = 1; j < n; j++) {
        if (!matrix[i][j]) {
          matrix[i][0] = matrix[0][j] = 0;
        }
      }
    }
    for (int i = m - 1; i >= 0; i--) {
      for (int j = 1; j < n; j++) {
        if (!matrix[i][0] || !matrix[0][j]) {
          matrix[i][j] = 0;
        }
      }
      if (flag_col0) {
        matrix[i][0] = 0;
      }
    }
  }
};

int main() {
  {
    vector<vector<int>> matrix{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}};
    vector<vector<int>> ans{{1, 0, 1}, {0, 0, 0}, {1, 0, 1}};
    Solution().setZeroes(matrix);
    assert(matrix == ans);
  }

  {
    vector<vector<int>> matrix{{0, 1, 2, 0}, {3, 4, 5, 2}, {1, 3, 1, 5}};
    vector<vector<int>> ans{{0, 0, 0, 0}, {0, 4, 5, 0}, {0, 3, 1, 0}};
    Solution().setZeroes(matrix);
    assert(matrix == ans);
  }
}