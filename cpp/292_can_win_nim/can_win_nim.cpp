/*
 * @Date: 2021-09-18 08:41:38
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-04
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  bool canWinNim(int n) { return n % 4 != 0; }
};

int main() {
  std::vector<std::tuple<int, bool>> tests{
      {4, false},
      {1, true},
      {2, true},
  };

  for (auto [n, ans] : tests) {
    assert(Solution().canWinNim(n) == ans);
  }
}