/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-29 18:50:01
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-29 18:50:21
 */

/*
dp[i][k][j] 表示利润
i 表示天数
k 表示允许操作的最大次数
j 表示持有状态 1 表示持有 0表示未持有

dp[i][k][0] = max(dp[i-1][k][0], dp[i-1][k][1] + prices[i])
dp[i][k][1] = max(dp[i-1][k][1], dp[i-1][k-1][0] - prices[i])

dp[0][k][0] = 0
第0天还没有开始 这时利润为0
dp[0][k][1] = -INT_MAX
第0天不可能持有，表示不可能状态
dp[i][0][0] = 0
不允许交易，利润是0
dp[i][0][1] = -INT_MAX
不允许交易，还持有股票，表示不可能状态
*/

#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

// dp[i][1][0] = max (dp[i-1][1][0], dp[i-1][1][1] + prices[i])
// dp[i][1][1] = max (dp[i-1][1][1], dp[i-1][0][0] - prices[i])
//             = max (dp[i-1][1][1], -prices[i]
// dp[i][0] = max (dp[i-1][0], dp[i-1][1] + prices[i])
// dp[i][1] = max (dp[i-1][1], -prices[i])
int max_profit_k_1(vector<int> prices) {
  {
    int n = prices.size();
    vector<vector<int>> dp(n, vector<int>(2, 0));
    dp[0][0] = 0;
    dp[0][1] = max(INT_MIN, -prices[0]);

    for (int i = 1; i < n; ++i) {
      dp[i][0] = max(dp[i - 1][0], dp[i - 1][1] + prices[i]);
      dp[i][1] = max(dp[i - 1][1], -prices[i]);
    }
    cout << dp[n - 1][0] << endl;
  }
  {
    int n = prices.size();
    int dp_i_0 = 0, dp_i_1 = INT_MIN;
    for (int i = 0; i < n; ++i) {
      dp_i_0 = max(dp_i_0, dp_i_1 + prices[i]);
      dp_i_1 = max(dp_i_1, -prices[i]);
    }
    cout << dp_i_0 << endl;
  }
  return 0;
}

// dp[i][k][0] = max (dp[i-1][k][0], dp[i-1][k][1] + prices[i])
// dp[i][k][1] = max (dp[i-1][k][1], dp[i-1][k - 1][0] - prices[i])
//             = max (dp[i-1][k][1], dp[i-1][k][1] - prices[i]
// dp[i][0] = max (dp[i-1][0], dp[i-1][1] + prices[i])
// dp[i][1] = max (dp[i-1][1], -prices[i])
int max_profit_k_inf(vector<int> prices) {}

int main() {
  vector<int> prices = {7, 1, 5, 3, 6, 4};
  max_profit_k_1(prices);
  prices = {7, 6, 4, 3, 1};
  max_profit_k_1(prices);
}
