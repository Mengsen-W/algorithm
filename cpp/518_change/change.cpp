/*
 * @Date: 2021-06-10 09:11:42
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-25
 */

#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int change(int amount, vector<int>& coins) {
    // dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 5]
    vector<int> dp(amount + 1, 0);  // dp[i]表示金额i的组合数
    dp[0] = 1;
    for (int coin : coins)
      for (int i = coin; i <= amount; ++i) dp[i] += dp[i - coin];
    return dp[amount];
  }
};

int main() {
  vector<tuple<int, vector<int>, int>> tests{
      {5, {1, 2, 5}, 4},
      {3, {2}, 0},
      {10, {10}, 1},
  };

  for (auto& [amount, coins, ans] : tests) {
    assert(Solution().change(amount, coins) == ans);
  }
}