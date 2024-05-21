/*
 * @Date: 2024-05-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-21
 * @FilePath: /algorithm/cpp/2769_the_maximum_achievable_x/the_maximum_achievable_x.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int theMaximumAchievableX(int num, int t) { return num + 2 * t; }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {4, 1, 6},
      {3, 2, 7},
  };

  for (auto &[num, t, ans] : tests) {
    assert(Solution().theMaximumAchievableX(num, t) == ans);
  }
}