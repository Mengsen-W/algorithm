/*
 * @Date: 2021-12-13 01:39:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-13 01:42:37
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxIncreaseKeepingSkyline(vector<vector<int>> grid) {
    int n = grid.size();
    vector<int> rowMax(n);
    vector<int> colMax(n);
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        rowMax[i] = max(rowMax[i], grid[i][j]);
        colMax[j] = max(colMax[j], grid[i][j]);
      }
    }
    int ans = 0;
    for (int i = 0; i < n; i++)
      for (int j = 0; j < n; j++) ans += min(rowMax[i], colMax[j]) - grid[i][j];
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{3, 0, 8, 4}, {2, 4, 5, 7}, {9, 2, 6, 3}, {0, 3, 1, 0}}, 35},
      {{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}, 0},
  };

  for (auto &[grid, ans] : tests) {
    assert(Solution().maxIncreaseKeepingSkyline(grid) == ans);
  }
}