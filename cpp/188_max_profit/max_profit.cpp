/*
 * @Date: 2023-10-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-04
 * @FilePath: /algorithm/cpp/188_max_profit/max_profit.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(int k, vector<int>& prices) {
    if (prices.empty()) {
      return 0;
    }

    int n = prices.size();
    k = min(k, n / 2);
    vector<int> buy(k + 1);
    vector<int> sell(k + 1);

    buy[0] = -prices[0];
    sell[0] = 0;
    for (int i = 1; i <= k; ++i) {
      buy[i] = sell[i] = INT_MIN / 2;
    }

    for (int i = 1; i < n; ++i) {
      buy[0] = max(buy[0], sell[0] - prices[i]);
      for (int j = 1; j <= k; ++j) {
        buy[j] = max(buy[j], sell[j] - prices[i]);
        sell[j] = max(sell[j], buy[j - 1] + prices[i]);
      }
    }

    return *max_element(sell.begin(), sell.end());
  }
};

int main() {
  vector<tuple<int, vector<int>, int>> tests{
      {2, {2, 4, 1}, 2},
      {2, {3, 2, 6, 5, 0, 3}, 7},
  };

  for (auto& [k, prices, ans] : tests) {
    assert(Solution().maxProfit(k, prices) == ans);
  }
}