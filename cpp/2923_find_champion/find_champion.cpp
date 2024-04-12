/*
 * @Date: 2024-04-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-12
 * @FilePath: /algorithm/cpp/2923_find_champion/find_champion.cpp
 */

#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findChampion(vector<vector<int>>& grid) {
    int n = grid.size();
    for (int i = 0; i < n; i++) {
      if (accumulate(grid[i].begin(), grid[i].end(), 0) == n - 1) {
        return i;
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{0, 1}, {0, 0}}, 0},
      {{{0, 0, 1}, {1, 0, 1}, {0, 0, 0}}, 1},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().findChampion(grid) == ans);
  }
}