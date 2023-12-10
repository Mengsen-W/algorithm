/*
 * @Date: 2023-12-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-10
 * @FilePath: /algorithm/cpp/70_climb_stairs/climb_stairs.cpp
 */

#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

class Solution {
 public:
  int climbStairs(int n) {
    double sqrt5 = sqrt(5);
    return (int)round(double(pow((1 + sqrt5) / 2, n + 1) - pow((1 - sqrt5) / 2, n + 1)) / sqrt5);
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {2, 2},
      {3, 3},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().climbStairs(n) == ans);
  }
}