/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 09:47:17
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-19 16:39:24
 */

#include <functional>
#include <unordered_map>
#include <vector>

int bottom_up(std::vector<int>& coins, int amount) {
  std::unordered_map<int, int> dict{};
  std::function<int(int)> dp = [&dp, &dict, &coins](int n) -> int {
    if (dict.count(n)) {
      return dict[n];
    }
    if (n == 0) return 0;
    if (n < 0) return -1;
    int res = INT_MAX;

    for (int coin : coins) {
      int subproblem = dp(n - coin);
      if (subproblem == -1) continue;
      res = std::min(res, 1 + subproblem);
    }
    dict[n] = res;
    return dict[n];
  };

  dp(amount);
}

int top_down(std::vector<int>& coins, int amount) {
  std::vector<int> dp(amount + 1, amount + 1);
  dp[0] = 0;

  for (int i = 0; i < dp.size(); ++i) {
    for (int coin : coins) {
      if (i - coin < 0) continue;
      dp[i] = std::min(dp[i], dp[i - coin] + 1);
    }
  }

  return (dp[amount] == amount + 1) ? -1 : dp[amount];
}

int main() {
  int a = 0;
  return 0;
}