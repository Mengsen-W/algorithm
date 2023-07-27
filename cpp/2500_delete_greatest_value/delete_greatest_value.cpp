/*
 * @Date: 2023-07-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-27
 * @FilePath: /algorithm/cpp/2500_delete_greatest_value/delete_greatest_value.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int deleteGreatestValue(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    for (int i = 0; i < m; i++) {
      sort(grid[i].begin(), grid[i].end());
    }
    int res = 0;
    for (int j = 0; j < n; j++) {
      int mx = 0;
      for (int i = 0; i < m; i++) {
        mx = max(mx, grid[i][j]);
      }
      res += mx;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2, 4}, {3, 3, 1}}, 8},
      {{{10}}, 10},
  };
  for (auto& [grid, ans] : tests) {
    assert(Solution().deleteGreatestValue(grid) == ans);
  }
}
