/*
 * @Date: 2024-02-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-28
 * @FilePath: /algorithm/cpp/2673_min_increments/min_increments.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minIncrements(int n, vector<int>& cost) {
    int ans = 0;
    for (int i = n - 2; i > 0; i -= 2) {
      ans += abs(cost[i] - cost[i + 1]);
      // 叶节点 i 和 i+1 的双亲节点下标为 i/2（整数除法）
      cost[i / 2] += max(cost[i], cost[i + 1]);
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<int>, int>> tests{
      {7, {1, 5, 2, 2, 3, 3, 1}, 6},
      {3, {5, 3, 3}, 0},
  };

  for (auto& [n, cost, ans] : tests) {
    assert(Solution().minIncrements(n, cost) == ans);
  }
}