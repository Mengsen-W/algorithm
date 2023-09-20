/*
 * @Date: 2023-09-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-20
 * @FilePath: /algorithm/cpp/LCP_06_min_count/min_count.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minCount(vector<int>& coins) {
    int sum = 0;
    for (int& i : coins) {
      sum += (i + 1) / 2;
    }
    return sum;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{4, 2, 1}, 4},
      {{2, 3, 10}, 8},
  };

  for (auto& [coins, ans] : tests) {
    assert(Solution().minCount(coins) == ans);
  }
}