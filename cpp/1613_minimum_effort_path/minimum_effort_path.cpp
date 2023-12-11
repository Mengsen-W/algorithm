/*
 * @Date: 2023-12-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-11
 * @FilePath: /algorithm/cpp/1613_minimum_effort_path/minimum_effort_path.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 private:
  static constexpr int dirs[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

 public:
  int minimumEffortPath(vector<vector<int>>& heights) {
    int m = heights.size();
    int n = heights[0].size();

    auto tupleCmp = [](const auto& e1, const auto& e2) {
      auto&& [x1, y1, d1] = e1;
      auto&& [x2, y2, d2] = e2;
      return d1 > d2;
    };
    priority_queue<tuple<int, int, int>, vector<tuple<int, int, int>>, decltype(tupleCmp)> q(tupleCmp);
    q.emplace(0, 0, 0);

    vector<int> dist(m * n, INT_MAX);
    dist[0] = 0;
    vector<int> seen(m * n);

    while (!q.empty()) {
      auto [x, y, d] = q.top();
      q.pop();
      int id = x * n + y;
      if (seen[id]) {
        continue;
      }
      if (x == m - 1 && y == n - 1) {
        break;
      }
      seen[id] = 1;
      for (int i = 0; i < 4; ++i) {
        int nx = x + dirs[i][0];
        int ny = y + dirs[i][1];
        if (nx >= 0 && nx < m && ny >= 0 && ny < n &&
            max(d, abs(heights[x][y] - heights[nx][ny])) < dist[nx * n + ny]) {
          dist[nx * n + ny] = max(d, abs(heights[x][y] - heights[nx][ny]));
          q.emplace(nx, ny, dist[nx * n + ny]);
        }
      }
    }

    return dist[m * n - 1];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2, 2}, {3, 8, 2}, {5, 3, 5}}, 2},
      {{{1, 2, 3}, {3, 8, 4}, {5, 3, 5}}, 1},
      {{{1, 2, 1, 1, 1}, {1, 2, 1, 2, 1}, {1, 2, 1, 2, 1}, {1, 2, 1, 2, 1}, {1, 1, 1, 2, 1}}, 0},
  };

  for (auto& [heights, ans] : tests) {
    assert(Solution().minimumEffortPath(heights) == ans);
  }
}