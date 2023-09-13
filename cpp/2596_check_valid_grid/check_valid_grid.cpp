/*
 * @Date: 2023-09-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-13
 * @FilePath: /algorithm/cpp/2596_check_valid_grid/check_valid_grid.cpp
 */

#include <array>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkValidGrid(vector<vector<int>>& grid) {
    if (grid[0][0] != 0) {
      return false;
    }
    int n = grid.size();
    vector<array<int, 2>> indices(n * n);
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        indices[grid[i][j]] = {i, j};
      }
    }
    for (int i = 1; i < indices.size(); i++) {
      int dx = abs(indices[i][0] - indices[i - 1][0]);
      int dy = abs(indices[i][1] - indices[i - 1][1]);
      if (dx * dy != 2) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, bool>> test_cases{
      {{{0, 11, 16, 5, 20}, {17, 4, 19, 10, 15}, {12, 1, 8, 21, 6}, {3, 18, 23, 14, 9}, {24, 13, 2, 7, 22}}, true},
      {{{0, 3, 6}, {5, 8, 1}, {2, 7, 4}}, false},
  };

  for (auto& [grid, expected] : test_cases) {
    assert(Solution().checkValidGrid(grid) == expected);
  }
}
