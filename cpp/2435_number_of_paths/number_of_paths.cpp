#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfPaths(vector<vector<int>>& grid, int k) {
    typedef long long ll;
    const int MOD = 1e9 + 7;
    int m = grid.size();
    int n = grid[0].size();

    auto dp = vector(m + 1, vector(n + 1, vector<ll>(k)));

    for (int i = 1; i <= m; i++) {
      for (int j = 1; j <= n; j++) {
        if (i == 1 && j == 1) {
          dp[i][j][grid[0][0] % k] = 1;
          continue;
        }

        int value = grid[i - 1][j - 1] % k;
        for (int r = 0; r < k; r++) {
          int prevMod = (r - value + k) % k;
          dp[i][j][r] = (dp[i - 1][j][prevMod] + dp[i][j - 1][prevMod]) % MOD;
        }
      }
    }

    return dp[m][n][0];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{5, 2, 4}, {3, 0, 5}, {0, 7, 2}}, 3, 2},
      {{{0, 0}}, 5, 1},
      {{{7, 3, 4, 9}, {2, 3, 6, 2}, {2, 3, 7, 0}}, 1, 10},
  };

  for (auto& [grid, k, expect] : tests) {
    assert(Solution().numberOfPaths(grid, k) == expect);
  }
}