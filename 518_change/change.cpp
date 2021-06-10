/*
 * @Date: 2021-06-10 09:11:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-10 09:14:36
 */

#include <cassert>
#include <vector>
using namespace std;

int change(int amount, vector<int>& coins) {
  // dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 5]
  vector<int> dp(amount + 1, 0);  // dp[i]表示金额i的组合数
  dp[0] = 1;
  for (int coin : coins)
    for (int i = coin; i <= amount; ++i) dp[i] += dp[i - coin];
  return dp[amount];
}

int main() {
  {
    int amount = 5;
    vector<int> coins{1, 2, 5};
    int ans = 4;
    assert(change(amount, coins) == ans);
  }
  {
    int amount = 3;
    vector<int> coins{2};
    int ans = 0;
    assert(change(amount, coins) == ans);
  }
  {
    int amount = 10;
    vector<int> coins{10};
    int ans = 1;
    assert(change(amount, coins) == ans);
  }
}