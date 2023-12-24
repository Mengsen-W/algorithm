/*
 * @Date: 2023-12-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-24
 * @FilePath: /algorithm/cpp/1954_minimum_perimeter/minimum_perimeter.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  long long minimumPerimeter(long long neededApples) {
    long long left = 1, right = 100000, ans = 0;
    while (left <= right) {
      long long mid = (left + right) / 2;
      if (2 * mid * (mid + 1) * (mid * 2 + 1) >= neededApples) {
        ans = mid;
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }
    return ans * 8;
  }
};

int main() {
  std::vector<std::tuple<long long, long long>> tests{
      {1, 8},
      {13, 16},
      {1000000000, 5040},
  };

  for (auto& [neededApples, ans] : tests) {
    assert(Solution().minimumPerimeter(neededApples) == ans);
  }
}