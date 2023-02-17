/*
 * @Date: 2023-02-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-17
 * @FilePath: /algorithm/cpp/1139_largest1_bordered_square/largest1_bordered_square.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int largest1BorderedSquare(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> left(m + 1, vector<int>(n + 1));
    vector<vector<int>> up(m + 1, vector<int>(n + 1));
    int maxBorder = 0;
    for (int i = 1; i <= m; i++) {
      for (int j = 1; j <= n; j++) {
        if (grid[i - 1][j - 1] == 1) {
          left[i][j] = left[i][j - 1] + 1;
          up[i][j] = up[i - 1][j] + 1;
          int border = min(left[i][j], up[i][j]);
          while (left[i - border + 1][j] < border || up[i][j - border + 1] < border) {
            border--;
          }
          maxBorder = max(maxBorder, border);
        }
      }
    }
    return maxBorder * maxBorder;
  }
};

int main() {
  {
    vector<vector<int>> grid{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}};
    int ans = 2;
    assert(Solution().largest1BorderedSquare(grid) == ans);
  }

  {
    vector<vector<int>> grid{{1, 1, 0, 0}};
    int ans = 1;
    assert(Solution().largest1BorderedSquare(grid) == ans);
  }
}