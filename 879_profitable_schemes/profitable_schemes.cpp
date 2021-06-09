/*
 * @Date: 2021-06-09 08:34:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-09 08:53:45
 */

#include <cassert>
#include <vector>
using namespace std;

int profitableSchemes(int n, int minProfit, vector<int>& group,
                      vector<int>& profit) {
  vector<vector<int>> dp(n + 1, vector<int>(minProfit + 1));
  for (int i = 0; i <= n; i++) {
    dp[i][0] = 1;
  }
  int len = group.size(), MOD = (int)1e9 + 7;
  for (int i = 1; i <= len; i++) {
    int members = group[i - 1], earn = profit[i - 1];
    for (int j = n; j >= members; j--) {
      for (int k = minProfit; k >= 0; k--) {
        dp[j][k] = (dp[j][k] + dp[j - members][max(0, k - earn)]) % MOD;
      }
    }
  }
  return dp[n][minProfit];
}

int main() {
  {
    int n = 5;
    int minProfit = 3;
    vector<int> group{2, 2};
    vector<int> profit{2, 3};
    int ans = 2;
    assert(profitableSchemes(n, minProfit, group, profit) == ans);
  }
  {
    int n = 10;
    int minProfit = 5;
    vector<int> group{2, 3, 5};
    vector<int> profit{6, 7, 8};
    int ans = 7;
    assert(profitableSchemes(n, minProfit, group, profit) == ans);
  }
}