/*
 * @Date: 2022-07-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-06
 * @FilePath: /algorithm/cpp/741_cherry_pickup/cherry_pickup.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int cherryPickup(vector<vector<int>> grid) {
    int n = grid.size();
    vector<vector<int>> f(n, vector<int>(n, INT_MIN));
    f[0][0] = grid[0][0];
    for (int k = 1; k < n * 2 - 1; ++k) {
      for (int x1 = min(k, n - 1); x1 >= max(k - n + 1, 0); --x1) {
        for (int x2 = min(k, n - 1); x2 >= x1; --x2) {
          int y1 = k - x1, y2 = k - x2;
          if (grid[x1][y1] == -1 || grid[x2][y2] == -1) {
            f[x1][x2] = INT_MIN;
            continue;
          }
          int res = f[x1][x2];  // 都往右
          if (x1) {
            res = max(res, f[x1 - 1][x2]);  // 往下，往右
          }
          if (x2) {
            res = max(res, f[x1][x2 - 1]);  // 往右，往下
          }
          if (x1 && x2) {
            res = max(res, f[x1 - 1][x2 - 1]);  // 都往下
          }
          res += grid[x1][y1];
          if (x2 != x1) {  // 避免重复摘同一个樱桃
            res += grid[x2][y2];
          }
          f[x1][x2] = res;
        }
      }
    }
    return max(f.back().back(), 0);
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{0, 1, -1}, {1, 0, -1}, {1, 1, 1}}, 5},
      {{{1, 1, -1}, {1, -1, 1}, {-1, 1, 1}}, 0},
  };

  for (auto &[grid, ans] : tests) {
    assert(Solution().cherryPickup(grid) == ans);
  }
}