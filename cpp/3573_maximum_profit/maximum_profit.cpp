#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumProfit(vector<int>& prices, int k) {
    int n = prices.size();
    vector<vector<long long>> dp(k + 1, vector<long long>(3));
    // 初始化第 0 天的状态
    for (int j = 1; j <= k; j++) {
      dp[j][1] = -prices[0];
      dp[j][2] = prices[0];
    }
    for (int i = 1; i < n; i++) {
      for (int j = k; j > 0; j--) {
        dp[j][0] = max(dp[j][0], max(dp[j][1] + prices[i], dp[j][2] - prices[i]));
        dp[j][1] = max(dp[j][1], dp[j - 1][0] - prices[i]);
        dp[j][2] = max(dp[j][2], dp[j - 1][0] + prices[i]);
      }
    }

    return dp[k][0];
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 7, 9, 8, 2}, 2, 14},
      {{12, 16, 19, 19, 8, 1, 19, 13, 9}, 3, 36},
  };

  for (auto &[prices, k, expect] : tests) {
    assert(Solution().maximumProfit(prices, k) == expect);
  }
}