/*
 * @Date: 2024-02-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-03
 * @FilePath: /algorithm/cpp/1690_stone_game_vii/stone_game_vii.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int stoneGameVII(vector<int>& stones) {
    int n = stones.size();
    vector<int> sum(n + 1);
    vector<vector<int>> dp(n, vector<int>(n, 0));

    for (int i = 0; i < n; i++) {
      sum[i + 1] = sum[i] + stones[i];
    }
    for (int i = n - 2; i >= 0; i--) {
      for (int j = i + 1; j < n; j++) {
        dp[i][j] = max(sum[j + 1] - sum[i + 1] - dp[i + 1][j], sum[j] - sum[i] - dp[i][j - 1]);
      }
    }

    return dp[0][n - 1];
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{5, 3, 1, 4, 2}, 6},
      {{7, 90, 5, 1, 100, 10, 10, 2}, 122},
  };

  for (auto& [stones, ans] : tests) {
    assert(Solution().stoneGameVII(stones) == ans);
  }
}