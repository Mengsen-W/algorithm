/*
 * @Date: 2023-09-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-26
 * @FilePath: /algorithm/cpp/2582_pass_the_pillow/pass_the_pillow.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int passThePillow(int n, int time) {
    time %= (n - 1) * 2;
    return time < n ? time + 1 : n * 2 - time - 1;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {4, 5, 2},
      {3, 2, 3},
  };

  for (auto& [n, time, ans] : tests) {
    assert(Solution().passThePillow(n, time) == ans);
  }
}