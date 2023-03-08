/*
 * @Date: 2023-03-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-08
 * @FilePath: /algorithm/cpp/47_max_value/max_value.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxValue(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> f(2, vector<int>(n));
    for (int i = 0; i < m; ++i) {
      int pos = i % 2;
      for (int j = 0; j < n; ++j) {
        f[pos][j] = 0;
        if (i > 0) {
          f[pos][j] = max(f[pos][j], f[1 - pos][j]);
        }
        if (j > 0) {
          f[pos][j] = max(f[pos][j], f[pos][j - 1]);
        }
        f[pos][j] += grid[i][j];
      }
    }
    return f[(m - 1) % 2][n - 1];
  }
};

int main() {
  vector<vector<int>> grid{{1, 3, 1}, {1, 5, 1}, {4, 2, 1}};
  int ans = 12;
  assert(Solution().maxValue(grid) == ans);
}