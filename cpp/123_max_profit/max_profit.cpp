/*
 * @Date: 2023-10-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-03
 * @FilePath: /algorithm/cpp/123_max_profit/max_profit.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(vector<int>& prices) {
    int n = prices.size();
    int buy1 = -prices[0], sell1 = 0;
    int buy2 = -prices[0], sell2 = 0;
    for (int i = 1; i < n; ++i) {
      buy1 = max(buy1, -prices[i]);
      sell1 = max(sell1, buy1 + prices[i]);
      buy2 = max(buy2, sell1 - prices[i]);
      sell2 = max(sell2, buy2 + prices[i]);
    }
    return sell2;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 3, 5, 0, 0, 3, 1, 4}, 6},
      {{1, 2, 3, 4, 5}, 4},
      {{7, 6, 4, 3, 1}, 0},
      {{1}, 0},
  };

  for (auto& [prices, ans] : tests) {
    assert(Solution().maxProfit(prices) == ans);
  }
}
