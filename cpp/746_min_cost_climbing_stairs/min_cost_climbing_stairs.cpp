/*
 * @Date: 2023-12-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-17
 * @FilePath: /algorithm/cpp/746_min_cost_climbing_stairs/min_cost_climbing_stairs.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minCostClimbingStairs(vector<int>& cost) {
    int n = cost.size();
    int prev = 0, curr = 0;
    for (int i = 2; i <= n; i++) {
      int next = min(curr + cost[i - 1], prev + cost[i - 2]);
      prev = curr;
      curr = next;
    }
    return curr;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{10, 15, 20}, 15},
      {{1, 100, 1, 1, 1, 100, 1, 1, 100, 1}, 6},
  };

  for (auto& [cost, ans] : tests) {
    assert(Solution().minCostClimbingStairs(cost) == ans);
  }
}