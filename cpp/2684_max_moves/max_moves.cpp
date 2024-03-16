/*
 * @Date: 2024-03-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-16
 * @FilePath: /algorithm/cpp/2684_max_moves/max_moves.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxMoves(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    unordered_set<int> q, q2;
    for (int i = 0; i < m; i++) {
      q.insert(i);
    }
    for (int j = 1; j < n; j++) {
      q2.clear();
      for (int i : q) {
        for (int i2 = i - 1; i2 <= i + 1; i2++) {
          if (0 <= i2 && i2 < m && grid[i][j - 1] < grid[i2][j]) {
            q2.insert(i2);
          }
        }
      }
      swap(q, q2);
      if (q.empty()) {
        return j - 1;
      }
    }
    return n - 1;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> test{
      {{{2, 4, 3, 5}, {5, 4, 9, 3}, {3, 4, 2, 11}, {10, 9, 13, 15}}, 3},
      {{{3, 2, 4}, {2, 1, 9}, {1, 1, 7}}, 0},
  };

  for (auto& [grid, ans] : test) {
    assert(Solution().maxMoves(grid) == ans);
  }
}