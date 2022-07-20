/*
 * @Date: 2022-07-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-20
 * @FilePath: /algorithm/1260_shift_grid/shift_grid.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> shiftGrid(vector<vector<int>>& grid, int k) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> ret(m, vector<int>(n));
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        int index1 = (i * n + j + k) % (m * n);
        ret[index1 / n][index1 % n] = grid[i][j];
      }
    }
    return ret;
  }
};

int main() {
  {
    vector<vector<int>> grid{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    int k = 1;
    vector<vector<int>> ans{{9, 1, 2}, {3, 4, 5}, {6, 7, 8}};
    assert(Solution().shiftGrid(grid, k) == ans);
  }

  {
    vector<vector<int>> grid{{3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}, {12, 0, 21, 13}};
    int k = 4;
    vector<vector<int>> ans{{12, 0, 21, 13}, {3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}};
    assert(Solution().shiftGrid(grid, k) == ans);
  }

  {
    vector<vector<int>> grid{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    int k = 9;
    vector<vector<int>> ans{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    assert(Solution().shiftGrid(grid, k) == ans);
  }
}