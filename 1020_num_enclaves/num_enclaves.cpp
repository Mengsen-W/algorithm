/*
 * @Date: 2022-02-12 00:02:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-12 00:15:41
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class SolutionDFS {
 public:
  vector<vector<int>> dirs = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

  int numEnclaves(const vector<vector<int>>& grid) {
    this->m = grid.size();
    this->n = grid[0].size();
    this->visited = vector<vector<bool>>(m, vector<bool>(n, false));
    for (int i = 0; i < m; i++) {
      dfs(grid, i, 0);
      dfs(grid, i, n - 1);
    }
    for (int j = 1; j < n - 1; j++) {
      dfs(grid, 0, j);
      dfs(grid, m - 1, j);
    }
    int enclaves = 0;
    for (int i = 1; i < m - 1; i++) {
      for (int j = 1; j < n - 1; j++) {
        if (grid[i][j] == 1 && !visited[i][j]) {
          enclaves++;
        }
      }
    }
    return enclaves;
  }

  void dfs(const vector<vector<int>>& grid, int row, int col) {
    if (row < 0 || row >= m || col < 0 || col >= n || grid[row][col] == 0 ||
        visited[row][col]) {
      return;
    }
    visited[row][col] = true;
    for (auto& dir : dirs) {
      dfs(grid, row + dir[0], col + dir[1]);
    }
  }

 private:
  int m, n;
  vector<vector<bool>> visited;
};

class SolutionBFS {
 public:
  vector<vector<int>> dirs = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

  int numEnclaves(const vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<bool>> visited =
        vector<vector<bool>>(m, vector<bool>(n, false));
    queue<pair<int, int>> qu;
    for (int i = 0; i < m; i++) {
      if (grid[i][0] == 1) {
        visited[i][0] = true;
        qu.emplace(i, 0);
      }
      if (grid[i][n - 1] == 1) {
        visited[i][n - 1] = true;
        qu.emplace(i, n - 1);
      }
    }
    for (int j = 1; j < n - 1; j++) {
      if (grid[0][j] == 1) {
        visited[0][j] = true;
        qu.emplace(0, j);
      }
      if (grid[m - 1][j] == 1) {
        visited[m - 1][j] = true;
        qu.emplace(m - 1, j);
      }
    }
    while (!qu.empty()) {
      auto [currRow, currCol] = qu.front();
      qu.pop();
      for (auto& dir : dirs) {
        int nextRow = currRow + dir[0], nextCol = currCol + dir[1];
        if (nextRow >= 0 && nextRow < m && nextCol >= 0 && nextCol < n &&
            grid[nextRow][nextCol] == 1 && !visited[nextRow][nextCol]) {
          visited[nextRow][nextCol] = true;
          qu.emplace(nextRow, nextCol);
        }
      }
    }
    int enclaves = 0;
    for (int i = 1; i < m - 1; i++) {
      for (int j = 1; j < n - 1; j++) {
        if (grid[i][j] == 1 && !visited[i][j]) {
          enclaves++;
        }
      }
    }
    return enclaves;
  }
};

class SolutionDiffUnion {
 public:
  int numEnclaves(const vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    UnionFind uf(grid);
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (grid[i][j] == 1) {
          int index = i * n + j;
          if (j + 1 < n && grid[i][j + 1] == 1) {
            uf.uni(index, index + 1);
          }
          if (i + 1 < m && grid[i + 1][j] == 1) {
            uf.uni(index, index + n);
          }
        }
      }
    }
    int enclaves = 0;
    for (int i = 1; i < m - 1; i++) {
      for (int j = 1; j < n - 1; j++) {
        if (grid[i][j] == 1 && !uf.isOnEdge(i * n + j)) {
          enclaves++;
        }
      }
    }
    return enclaves;
  }

 private:
  class UnionFind {
   public:
    UnionFind(const vector<vector<int>>& grid) {
      int m = grid.size(), n = grid[0].size();
      this->parent = vector<int>(m * n);
      this->onEdge = vector<bool>(m * n, false);
      this->rank = vector<int>(m * n);
      for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
          if (grid[i][j] == 1) {
            int index = i * n + j;
            parent[index] = index;
            if (i == 0 || i == m - 1 || j == 0 || j == n - 1) {
              onEdge[index] = true;
            }
          }
        }
      }
    }

    int find(int i) {
      if (parent[i] != i) {
        parent[i] = find(parent[i]);
      }
      return parent[i];
    }

    void uni(int x, int y) {
      int rootx = find(x);
      int rooty = find(y);
      if (rootx != rooty) {
        if (rank[rootx] > rank[rooty]) {
          parent[rooty] = rootx;
          onEdge[rootx] = onEdge[rootx] | onEdge[rooty];
        } else if (rank[rootx] < rank[rooty]) {
          parent[rootx] = rooty;
          onEdge[rooty] = onEdge[rooty] | onEdge[rootx];
        } else {
          parent[rooty] = rootx;
          onEdge[rootx] = onEdge[rootx] | onEdge[rooty];
          rank[rootx]++;
        }
      }
    }

    bool isOnEdge(int i) { return onEdge[find(i)]; }

   private:
    vector<int> parent;
    vector<bool> onEdge;
    vector<int> rank;
  };
};

int main() {
  {
    vector<vector<int>> grid{
        {0, 0, 0, 0}, {1, 0, 1, 0}, {0, 1, 1, 0}, {0, 0, 0, 0}};
    assert(SolutionDFS().numEnclaves(grid) == 3);
    assert(SolutionBFS().numEnclaves(grid) == 3);
    assert(SolutionDiffUnion().numEnclaves(grid) == 3);
  }
  {
    vector<vector<int>> grid{
        {0, 1, 1, 0}, {0, 0, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 0}};
    assert(SolutionDFS().numEnclaves(grid) == 0);
    assert(SolutionBFS().numEnclaves(grid) == 0);
    assert(SolutionDiffUnion().numEnclaves(grid) == 0);
  }
}