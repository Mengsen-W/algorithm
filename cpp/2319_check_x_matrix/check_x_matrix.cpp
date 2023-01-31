/*
 * @Date: 2023-01-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-31
 * @FilePath: /algorithm/cpp/2319_check_x_matrix/check_x_matrix.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkXMatrix(vector<vector<int>>& grid) {
    int n = grid.size();
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        if (i == j || (i + j) == (n - 1)) {
          if (grid[i][j] == 0) {
            return false;
          }
        } else if (grid[i][j]) {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  {
    vector<vector<int>> grid{{2, 0, 0, 1}, {0, 3, 1, 0}, {0, 5, 2, 0}, {4, 0, 0, 2}};
    bool ans = true;
    assert(Solution().checkXMatrix(grid) == ans);
  }

  {
    vector<vector<int>> grid{{5, 7, 0}, {0, 3, 1}, {0, 5, 0}};
    bool ans = false;
    assert(Solution().checkXMatrix(grid) == ans);
  }
}