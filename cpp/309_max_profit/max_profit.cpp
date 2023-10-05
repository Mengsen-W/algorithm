/*
 * @Date: 2023-10-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-05
 * @FilePath: /algorithm/cpp/309_max_profit/max_profit.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(vector<int>& prices) {
    if (prices.empty()) return 0;
    int n = prices.size(), f0 = -prices[0], f1 = 0, f2 = 0;
    for (int i = 1; i < n; ++i) tie(f0, f1, f2) = make_tuple(max(f0, f2 - prices[i]), f0 + prices[i], max(f1, f2));
    return max(f1, f2);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 0, 2}, 3},
      {{1}, 0},
  };

  for (auto &[prices, ans] : tests) {
    assert(Solution().maxProfit(prices) == ans);
  }
}