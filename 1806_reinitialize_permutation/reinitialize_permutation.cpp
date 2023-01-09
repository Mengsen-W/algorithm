/*
 * @Date: 2023-01-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-09
 * @FilePath: /algorithm/1806_reinitialize_permutation/reinitialize_permutation.cpp
 */

#include <cassert>

class Solution {
 public:
  int reinitializePermutation(int n) {
    if (n == 2) {
      return 1;
    }
    int step = 1, pow2 = 2;
    while (pow2 != 1) {
      step++;
      pow2 = pow2 * 2 % (n - 1);
    }
    return step;
  }
};

int main() {
  {
    int n = 2;
    int ans = 1;
    assert(Solution().reinitializePermutation(n) == ans);
  }

  {
    int n = 4;
    int ans = 2;
    assert(Solution().reinitializePermutation(n) == ans);
  }

  {
    int n = 6;
    int ans = 4;
    assert(Solution().reinitializePermutation(n) == ans);
  }
}