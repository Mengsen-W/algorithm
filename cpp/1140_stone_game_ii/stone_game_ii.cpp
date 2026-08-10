#include <cassert>
#include <cstring>
#include <functional>
#include <tuple>
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
  vector<tuple<vector<int>, int>> tests{
      {{2, 7, 9, 4, 4}, 10},
      {{1, 2, 3, 4, 5, 100}, 104},
  };

  for (auto& [piles, expected] : tests) {
    assert(Solution().stoneGameII(piles) == expected);
  }
  return 0;
}