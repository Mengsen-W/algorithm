/*
 * @Date: 2024-03-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-22
 * @FilePath: /algorithm/cpp/2617_minimum_visited_cells/minimum_visited_cells.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumVisitedCells(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> dist(m, vector<int>(n, -1));
    dist[0][0] = 1;
    vector<priority_queue<pair<int, int>, vector<pair<int, int>>, greater<>>> row(m), col(n);

    auto update = [](int& x, int y) {
      if (x == -1 || y < x) {
        x = y;
      }
    };

    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        while (!row[i].empty() && row[i].top().second + grid[i][row[i].top().second] < j) {
          row[i].pop();
        }
        if (!row[i].empty()) {
          update(dist[i][j], dist[i][row[i].top().second] + 1);
        }

        while (!col[j].empty() && col[j].top().second + grid[col[j].top().second][j] < i) {
          col[j].pop();
        }
        if (!col[j].empty()) {
          update(dist[i][j], dist[col[j].top().second][j] + 1);
        }
        if (dist[i][j] != -1) {
          row[i].emplace(dist[i][j], j);
          col[j].emplace(dist[i][j], i);
        }
      }
    }
    return dist[m - 1][n - 1];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{3, 4, 2, 1}, {4, 2, 3, 1}, {2, 1, 0, 0}, {2, 4, 0, 0}}, 4},
      {{{3, 4, 2, 1}, {4, 2, 1, 1}, {2, 1, 1, 0}, {3, 4, 1, 0}}, 3},
      {{{2, 1, 0}, {1, 0, 0}}, -1},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().minimumVisitedCells(grid) == ans);
  }
}