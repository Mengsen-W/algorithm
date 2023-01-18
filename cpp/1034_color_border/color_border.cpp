/*
 * @Date: 2021-12-07 00:56:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-07 01:19:25
 */

#include <cassert>
#include <iostream>
#include <queue>
#include <vector>

using namespace std;

typedef pair<int, int> pii;

class Solution {
 public:
  vector<vector<int>> colorBorder_bfs(vector<vector<int>>& grid, int row,
                                      int col, int color) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<bool>> visited(m, vector<bool>(n, false));
    vector<pii> borders;
    int originalColor = grid[row][col];
    int direc[4][2] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
    queue<pii> q;
    q.emplace(row, col);
    visited[row][col] = true;
    while (!q.empty()) {
      pii& node = q.front();
      q.pop();
      int x = node.first, y = node.second;

      bool isBorder = false;
      for (int i = 0; i < 4; i++) {
        int nx = direc[i][0] + x, ny = direc[i][1] + y;
        if (!(nx >= 0 && nx < m && ny >= 0 && ny < n &&
              grid[nx][ny] == originalColor)) {
          isBorder = true;
        } else if (!visited[nx][ny]) {
          visited[nx][ny] = true;
          q.emplace(nx, ny);
        }
      }
      if (isBorder) {
        borders.emplace_back(x, y);
      }
    }
    for (auto& [x, y] : borders) {
      grid[x][y] = color;
    }
    return grid;
  }

  vector<vector<int>> colorBorder_dfs(vector<vector<int>>& grid, int row,
                                      int col, int color) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<bool>> visited(m, vector<bool>(n, false));
    vector<pii> borders;
    int originalColor = grid[row][col];
    visited[row][col] = true;
    dfs(grid, row, col, visited, borders, originalColor);
    for (auto& [x, y] : borders) {
      grid[x][y] = color;
    }
    return grid;
  }

  void dfs(vector<vector<int>>& grid, int x, int y,
           vector<vector<bool>>& visited, vector<pii>& borders,
           int originalColor) {
    int m = grid.size(), n = grid[0].size();
    bool isBorder = false;
    int direc[4][2] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
    for (int i = 0; i < 4; i++) {
      int nx = direc[i][0] + x, ny = direc[i][1] + y;
      if (!(nx >= 0 && nx < m && ny >= 0 && ny < n &&
            grid[nx][ny] == originalColor)) {
        isBorder = true;
      } else if (!visited[nx][ny]) {
        visited[nx][ny] = true;
        dfs(grid, nx, ny, visited, borders, originalColor);
      }
    }
    if (isBorder) {
      borders.emplace_back(x, y);
    }
  }
};

int main() {
  {
    vector<vector<int>> grid = {{1, 1}, {1, 2}};
    assert((Solution().colorBorder_bfs(grid, 0, 0, 3) ==
            vector<vector<int>>{{3, 3}, {3, 2}}));
  }
  {
    vector<vector<int>> grid = {{1, 1}, {1, 2}};
    assert((Solution().colorBorder_dfs(grid, 0, 0, 3) ==
            vector<vector<int>>{{3, 3}, {3, 2}}));
  }
  {
    vector<vector<int>> grid = {{1, 2, 2}, {2, 3, 2}};
    assert((Solution().colorBorder_bfs(grid, 0, 1, 3) ==
            vector<vector<int>>{{1, 3, 3}, {2, 3, 3}}));
  }
  {
    vector<vector<int>> grid = {{1, 2, 2}, {2, 3, 2}};
    assert((Solution().colorBorder_dfs(grid, 0, 1, 3) ==
            vector<vector<int>>{{1, 3, 3}, {2, 3, 3}}));
  }
  {
    vector<vector<int>> grid = {{1, 1, 1}, {1, 1, 1}, {1, 1, 1}};
    assert((Solution().colorBorder_bfs(grid, 1, 1, 2) ==
            vector<vector<int>>{{2, 2, 2}, {2, 1, 2}, {2, 2, 2}}));
  }
  {
    vector<vector<int>> grid = {{1, 1, 1}, {1, 1, 1}, {1, 1, 1}};
    assert((Solution().colorBorder_dfs(grid, 1, 1, 2) ==
            vector<vector<int>>{{2, 2, 2}, {2, 1, 2}, {2, 2, 2}}));
  }
}