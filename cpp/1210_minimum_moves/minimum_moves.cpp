/*
 * @Date: 2023-02-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-05
 * @FilePath: /algorithm/cpp/1210_minimum_moves/minimum_moves.cpp
 */

#include <array>
#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumMoves(vector<vector<int>>& grid) {
    int n = grid.size();
    vector<vector<array<int, 2>>> dist(n, vector<array<int, 2>>(n, {-1, -1}));
    dist[0][0][0] = 0;
    queue<tuple<int, int, int>> q;
    q.emplace(0, 0, 0);

    while (!q.empty()) {
      auto [x, y, status] = q.front();
      q.pop();
      if (status == 0) {
        // 向右移动一个单元格
        if (y + 2 < n && dist[x][y + 1][0] == -1 && grid[x][y + 2] == 0) {
          dist[x][y + 1][0] = dist[x][y][0] + 1;
          q.emplace(x, y + 1, 0);
        }
        // 向下移动一个单元格
        if (x + 1 < n && dist[x + 1][y][0] == -1 && grid[x + 1][y] == 0 && grid[x + 1][y + 1] == 0) {
          dist[x + 1][y][0] = dist[x][y][0] + 1;
          q.emplace(x + 1, y, 0);
        }
        // 顺时针旋转 90 度
        if (x + 1 < n && y + 1 < n && dist[x][y][1] == -1 && grid[x + 1][y] == 0 && grid[x + 1][y + 1] == 0) {
          dist[x][y][1] = dist[x][y][0] + 1;
          q.emplace(x, y, 1);
        }
      } else {
        // 向右移动一个单元格
        if (y + 1 < n && dist[x][y + 1][1] == -1 && grid[x][y + 1] == 0 && grid[x + 1][y + 1] == 0) {
          dist[x][y + 1][1] = dist[x][y][1] + 1;
          q.emplace(x, y + 1, 1);
        }
        // 向下移动一个单元格
        if (x + 2 < n && dist[x + 1][y][1] == -1 && grid[x + 2][y] == 0) {
          dist[x + 1][y][1] = dist[x][y][1] + 1;
          q.emplace(x + 1, y, 1);
        }
        // 逆时针旋转 90 度
        if (x + 1 < n && y + 1 < n && dist[x][y][0] == -1 && grid[x][y + 1] == 0 && grid[x + 1][y + 1] == 0) {
          dist[x][y][0] = dist[x][y][1] + 1;
          q.emplace(x, y, 0);
        }
      }
    }

    return dist[n - 1][n - 2][0];
  }
};

int main() {
  {
    vector<vector<int>> grid{{0, 0, 0, 0, 0, 1}, {1, 1, 0, 0, 1, 0}, {0, 0, 0, 0, 1, 1},
                             {0, 0, 1, 0, 1, 0}, {0, 1, 1, 0, 0, 0}, {0, 1, 1, 0, 0, 0}};
    int ans = 11;
    assert(Solution().minimumMoves(grid) == ans);
  }

  {
    vector<vector<int>> grid{{0, 0, 1, 1, 1, 1}, {0, 0, 0, 0, 1, 1}, {1, 1, 0, 0, 0, 1},
                             {1, 1, 1, 0, 0, 1}, {1, 1, 1, 0, 0, 1}, {1, 1, 1, 0, 0, 0}};
    int ans = 9;
    assert(Solution().minimumMoves(grid) == ans);
  }
}