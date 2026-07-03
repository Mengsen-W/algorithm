#include <cassert>
#include <climits>
#include <deque>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
  static constexpr int DIRS[4][2] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

 public:
  bool findSafeWalk(vector<vector<int>>& grid, int health) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> dis(m, vector<int>(n, INT_MAX));

    deque<pair<int, int>> q;
    q.emplace_front(0, 0);
    dis[0][0] = grid[0][0];

    while (!q.empty()) {
      auto [cx, cy] = q.front();
      q.pop_front();
      // 第一次出队时，保证是最短距离
      if (cx == m - 1 && cy == n - 1) {
        return true;
      }

      for (auto& [dx, dy] : DIRS) {
        int nx = cx + dx, ny = cy + dy;
        if (nx < 0 || ny < 0 || nx >= m || ny >= n) {
          continue;
        }
        int cost = dis[cx][cy] + grid[nx][ny];
        // 剪枝：新距离不满足健康要求
        if (cost >= health) {
          continue;
        }
        if (cost < dis[nx][ny]) {
          dis[nx][ny] = cost;
          if (grid[nx][ny] == 0) {
            q.emplace_front(nx, ny);
          } else {
            q.emplace_back(nx, ny);
          }
        }
      }
    }

    return false;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, bool>> tests{
      {{{0, 1, 0, 0, 0}, {0, 1, 0, 1, 0}, {0, 0, 0, 1, 0}}, 1, true},
      {{{0, 1, 1, 0, 0, 0}, {1, 0, 1, 0, 0, 0}, {0, 1, 1, 1, 0, 1}, {0, 0, 1, 0, 1, 0}}, 3, false},
      {{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}, 5, true},
  };

  for (auto& [grid, health, ans] : tests) {
    assert(Solution().findSafeWalk(grid, health) == ans);
  }
}