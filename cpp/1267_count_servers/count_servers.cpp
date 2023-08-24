/*
 * @Date: 2023-08-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-24
 * @FilePath: /algorithm/cpp/1267_count_servers/count_servers.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int countServers(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    unordered_map<int, int> rows, cols;

    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        if (grid[i][j] == 1) {
          ++rows[i];
          ++cols[j];
        }
      }
    }
    int ans = 0;
    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        if (grid[i][j] == 1 && (rows[i] > 1 || cols[j] > 1)) {
          ++ans;
        }
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> test_cases{
      {{{1, 0}, {0, 1}}, 0},
      {{{1, 0}, {1, 1}}, 3},
      {{{1, 1, 0, 0}, {0, 0, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 1}}, 4},
  };

  for (auto& [grid, ans] : test_cases) {
    assert(Solution().countServers(grid) == ans);
  }
}