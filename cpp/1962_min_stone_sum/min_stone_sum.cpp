/*
 * @Date: 2023-12-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-23
 * @FilePath: /algorithm/cpp/1962_min_stone_sum/min_stone_sum.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minStoneSum(vector<int>& piles, int k) {
    priority_queue<int> pq(piles.begin(), piles.end());
    for (int i = 0; i < k; i++) {
      int pile = pq.top();
      pq.pop();
      pile -= pile / 2;
      pq.push(pile);
    }
    int sum = 0;
    while (!pq.empty()) {
      sum += pq.top();
      pq.pop();
    }
    return sum;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{5, 4, 9}, 2, 12},
      {{4, 3, 6, 7}, 3, 12},
  };

  for (auto& [piles, k, ans] : tests) {
    assert(Solution().minStoneSum(piles, k) == ans);
  }
}