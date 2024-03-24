/*
 * @Date: 2024-03-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-24
 * @FilePath: /algorithm/cpp/322_coin_change/coin_change.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int coinChange(vector<int>& coins, int amount) {
    int Max = amount + 1;
    vector<int> dp(amount + 1, Max);
    dp[0] = 0;
    for (int i = 1; i <= amount; ++i) {
      for (int j = 0; j < (int)coins.size(); ++j) {
        if (coins[j] <= i) {
          dp[i] = min(dp[i], dp[i - coins[j]] + 1);
        }
      }
    }
    return dp[amount] > amount ? -1 : dp[amount];
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 5}, 11, 3},
      {{2}, 3, -1},
      {{1}, 0, 0},
  };

  for (auto &[coins, amount, ans] : tests) {
    assert(Solution().coinChange(coins, amount) == ans);
  }
}