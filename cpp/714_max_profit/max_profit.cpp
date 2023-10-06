/*
 * @Date: 2023-10-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-06
 * @FilePath: /algorithm/cpp/714_max_profit/max_profit.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(vector<int>& prices, int fee) {
    int n = prices.size();
    int buy = prices[0] + fee;
    int profit = 0;
    for (int i = 1; i < n; ++i) {
      if (prices[i] + fee < buy) {
        buy = prices[i] + fee;
      } else if (prices[i] > buy) {
        profit += prices[i] - buy;
        buy = prices[i];
      }
    }
    return profit;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 3, 2, 8, 4, 9}, 2, 8},
      {{1, 3, 7, 5, 10, 3}, 3, 6},
  };

  for (auto& [prices, fee, ans] : tests) {
    assert(Solution().maxProfit(prices, fee) == ans);
  }
}