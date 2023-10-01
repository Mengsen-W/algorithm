/*
 * @Date: 2023-10-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-01
 * @FilePath: /algorithm/cpp/121_max_profit/max_profit.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(vector<int>& prices) {
    int inf = 1e9;
    int minprice = inf, maxprofit = 0;
    for (int price : prices) {
      maxprofit = max(maxprofit, price - minprice);
      minprice = min(price, minprice);
    }
    return maxprofit;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{7, 1, 5, 3, 6, 4}, 5},
      {{7, 6, 4, 3, 1}, 0},
  };

  for (auto& [prices, ans] : tests) {
    assert(Solution().maxProfit(prices) == ans);
  }
}