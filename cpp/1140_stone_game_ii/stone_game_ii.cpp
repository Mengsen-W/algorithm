/*
 * @Date: 2023-02-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-22
 * @FilePath: /algorithm/cpp/1140_stone_game_ii/stone_game_ii.cpp
 */

#include <cassert>
#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  int stoneGameII(vector<int>& piles) {
    int n = piles.size();
    int s[n + 1];
    s[0] = 0;
    for (int i = 0; i < n; ++i) {
      s[i + 1] = s[i] + piles[i];
    }
    int f[n][n + 1];
    memset(f, 0, sizeof f);
    function<int(int, int)> dfs = [&](int i, int m) -> int {
      if (m * 2 >= n - i) {
        return s[n] - s[i];
      }
      if (f[i][m]) {
        return f[i][m];
      }
      int res = 0;
      for (int x = 1; x <= m << 1; ++x) {
        res = max(res, s[n] - s[i] - dfs(i + x, max(x, m)));
      }
      return f[i][m] = res;
    };
    return dfs(0, 1);
  }
};

int main() {
  {
    vector<int> piles{2, 7, 9, 4, 4};
    int ans = 10;
    assert(Solution().stoneGameII(piles) == ans);
  }

  {
    vector<int> piles{1, 2, 3, 4, 5, 100};
    int ans = 104;
    assert(Solution().stoneGameII(piles) == ans);
  }
}