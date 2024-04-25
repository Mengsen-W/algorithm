/*
 * @Date: 2024-04-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-25
 * @FilePath: /algorithm/cpp/2739_distance_traveled/distance_traveled.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int distanceTraveled(int mainTank, int additionalTank) {
    int ans = 0;
    while (mainTank >= 5) {
      mainTank -= 5;
      ans += 50;
      if (additionalTank > 0) {
        additionalTank--;
        mainTank++;
      }
    }
    return ans + mainTank * 10;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {5, 10, 60},
      {1, 2, 10},
  };

  for (auto& [mainTank, additionalTank, ans] : tests) {
    assert(Solution().distanceTraveled(mainTank, additionalTank) == ans);
  }
}
