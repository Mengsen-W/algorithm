/*
 * @Date: 2023-08-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-04
 * @FilePath: /algorithm/cpp/980_unique_paths_iii/unique_paths_iii.cpp
 */

#include <array>
#include <cassert>
#include <functional>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int uniquePathsIII(vector<vector<int>>& grid) {
    int r = grid.size(), c = grid[0].size();
    int si = 0, sj = 0, st = 0;
    unordered_map<int, int> memo;
    for (int i = 0; i < r; i++) {
      for (int j = 0; j < c; j++) {
        if (grid[i][j] == 0 || grid[i][j] == 2) {
          st |= (1 << (i * c + j));
        } else if (grid[i][j] == 1) {
          si = i, sj = j;
        }
      }
    }

    function<int(int, int, int)> dp = [&](int i, int j, int st) -> int {
      if (grid[i][j] == 2) {
        if (st == 0) {
          return 1;
        }
        return 0;
      }
      int key = ((i * c + j) << (r * c)) + st;
      if (!memo.count(key)) {
        int res = 0;
        vector<array<int, 2>> dir({{-1, 0}, {1, 0}, {0, -1}, {0, 1}});
        for (auto& [di, dj] : dir) {
          int ni = i + di, nj = j + dj;
          if (ni >= 0 && ni < r && nj >= 0 && nj < c && (st & (1 << (ni * c + nj))) > 0) {
            res += dp(ni, nj, st ^ (1 << (ni * c + nj)));
          }
        }
        memo[key] = res;
      }
      return memo[key];
    };
    return dp(si, sj, st);
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 0, 0, 0}, {0, 0, 0, 0}, {0, 0, 2, -1}}, 2},
      {{{1, 0, 0, 0}, {0, 0, 0, 0}, {0, 0, 0, 2}}, 4},
      {{{0, 1}, {2, 0}}, 0},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().uniquePathsIII(grid) == ans);
  }
}