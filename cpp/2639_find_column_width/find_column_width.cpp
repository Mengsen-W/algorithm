/*
 * @Date: 2024-04-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-27
 * @FilePath: /algorithm/cpp/2639_find_column_width/find_column_width.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findColumnWidth(vector<vector<int>>& grid) {
    int n = grid.size(), m = grid[0].size();
    vector<int> res(m);
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < m; j++) {
        int x = grid[i][j];
        int length = 0;
        if (x <= 0) {
          length = 1;
        }
        while (x != 0) {
          length += 1;
          x /= 10;
        }
        res[j] = max(res[j], length);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{1}, {22}, {333}}, {3}},
      {{{-15, 1, 3}, {15, 7, 12}, {5, 6, -2}}, {3, 1, 2}},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().findColumnWidth(grid) == ans);
  }
}