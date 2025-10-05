#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

static const int dirs[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

class Solution {
 public:
  vector<vector<int>> heights;

  void bfs(int row, int col, vector<vector<bool>>& ocean) {
    if (ocean[row][col]) {
      return;
    }
    int m = heights.size();
    int n = heights[0].size();
    ocean[row][col] = true;
    queue<pair<int, int>> qu;
    qu.emplace(row, col);
    while (!qu.empty()) {
      auto [row, col] = qu.front();
      qu.pop();
      for (int i = 0; i < 4; i++) {
        int newRow = row + dirs[i][0], newCol = col + dirs[i][1];
        if (newRow >= 0 && newRow < m && newCol >= 0 && newCol < n && heights[newRow][newCol] >= heights[row][col] &&
            !ocean[newRow][newCol]) {
          ocean[newRow][newCol] = true;
          qu.emplace(newRow, newCol);
        }
      }
    }
  }

  vector<vector<int>> pacificAtlantic(vector<vector<int>>& heights) {
    this->heights = heights;
    int m = heights.size();
    int n = heights[0].size();
    vector<vector<bool>> pacific(m, vector<bool>(n, false));
    vector<vector<bool>> atlantic(m, vector<bool>(n, false));

    for (int i = 0; i < m; i++) {
      bfs(i, 0, pacific);
    }
    for (int j = 1; j < n; j++) {
      bfs(0, j, pacific);
    }
    for (int i = 0; i < m; i++) {
      bfs(i, n - 1, atlantic);
    }
    for (int j = 0; j < n - 1; j++) {
      bfs(m - 1, j, atlantic);
    }
    vector<vector<int>> result;
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (pacific[i][j] && atlantic[i][j]) {
          vector<int> cell;
          cell.emplace_back(i);
          cell.emplace_back(j);
          result.emplace_back(cell);
        }
      }
    }
    return result;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>>> tests{
      {
          {{1, 2, 2, 3, 5}, {3, 2, 3, 4, 4}, {2, 4, 5, 3, 1}, {6, 7, 1, 4, 5}, {5, 1, 1, 2, 4}},
          {{0, 4}, {1, 3}, {1, 4}, {2, 2}, {3, 0}, {3, 1}, {4, 0}},
      },
      {
          {{2, 1}, {1, 2}},
          {{0, 0}, {0, 1}, {1, 0}, {1, 1}},
      },
  };

  for (auto& [heights, ans] : tests) {
    assert(Solution().pacificAtlantic(heights) == ans);
  }
}