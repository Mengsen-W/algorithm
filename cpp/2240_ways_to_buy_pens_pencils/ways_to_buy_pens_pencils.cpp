/*
 * @Date: 2023-09-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-01
 * @FilePath: /algorithm/cpp/2240_ways_to_buy_pens_pencils/ways_to_buy_pens_pencils.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  long long waysToBuyPensPencils(int total, int cost1, int cost2) {
    // 走小的
    if (cost1 < cost2) std::swap(cost1, cost2);
    long long ans = 0;
    for (int s = 0; s <= total; s += cost1) ans += 1 + (total - s) / cost2;
    return ans;
  }
};

int main() {
  std::vector<std::tuple<int, int, int, long long>> tests{
      {20, 10, 5, 9},
      {5, 10, 10, 1},
  };

  for (auto &[total, cost1, cost2, expected] : tests) {
    assert(Solution().waysToBuyPensPencils(total, cost1, cost2) == expected);
  }
}