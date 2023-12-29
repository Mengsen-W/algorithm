/*
 * @Date: 2023-12-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-29
 * @FilePath: /algorithm/cpp/2706_buy_choco/buy_choco.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int buyChoco(vector<int>& prices, int money) {
    int fi = INT_MAX, se = INT_MAX;
    for (auto p : prices) {
      if (p < fi) {
        se = fi;
        fi = p;
      } else if (p < se) {
        se = p;
      }
    }
    return money < fi + se ? money : money - fi - se;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 2}, 3, 0},
      {{3, 2, 3}, 3, 3},
  };

  for (auto& [prices, money, ans] : tests) {
    assert(Solution().buyChoco(prices, money) == ans);
  }
}