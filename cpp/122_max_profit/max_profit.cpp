/*
 * @Date: 2023-10-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-02
 * @FilePath: /algorithm/cpp/122_max_profit/max_profit.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(vector<int>& prices) {
    int ans = 0;
    int n = prices.size();
    for (int i = 1; i < n; ++i) {
      ans += max(0, prices[i] - prices[i - 1]);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{7, 1, 5, 3, 6, 4}, 7},
      {{1, 2, 3, 4, 5}, 4},
      {{7, 6, 4, 3, 1}, 0},
  };

  for (auto& [prices, ans] : tests) {
    assert(Solution().maxProfit(prices) == ans);
  }
}
