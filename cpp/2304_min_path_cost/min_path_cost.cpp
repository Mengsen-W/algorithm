/*
 * @Date: 2023-11-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-22
 * @FilePath: /algorithm/cpp/2304_min_path_cost/min_path_cost.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minPathCost(vector<vector<int>>& grid, vector<vector<int>>& moveCost) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> dp(2, vector<int>(n));
    dp[0] = grid[0];
    int cur = 0;
    for (int i = 1; i < m; i++) {
      int next = 1 - cur;
      for (int j = 0; j < n; j++) {
        dp[next][j] = INT_MAX;
        for (int k = 0; k < n; k++) {
          dp[next][j] = min(dp[next][j], dp[cur][k] + moveCost[grid[i - 1][k]][j] + grid[i][j]);
        }
      }
      cur = next;
    }
    return *min_element(dp[cur].begin(), dp[cur].end());
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>, int>> tests{
      {{{5, 3}, {4, 0}, {2, 1}}, {{9, 8}, {1, 5}, {10, 12}, {18, 6}, {2, 4}, {14, 3}}, 17},
      {{{5, 1, 2}, {4, 0, 3}}, {{12, 10, 15}, {20, 23, 8}, {21, 7, 1}, {8, 1, 13}, {9, 10, 25}, {5, 3, 2}}, 6},
  };

  for (auto& [grid, moveCost, ans] : tests) {
    assert(Solution().minPathCost(grid, moveCost) == ans);
  }
}