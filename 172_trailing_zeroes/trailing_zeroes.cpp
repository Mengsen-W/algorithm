/*
 * @Date: 2022-03-24 23:04:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-24 23:18:51
 * @FilePath: /algorithm/172_trailing_zeroes/trailing_zeroes.cpp
 */

#include <cassert>

class Solution {
 public:
  int trailingZeroes(int n) {
    int ans = 0;
    while (n) {
      n /= 5;
      ans += n;
    }
    return ans;
  }
};

int main() {
  assert(Solution().trailingZeroes(3) == 0);
  assert(Solution().trailingZeroes(5) == 1);
  assert(Solution().trailingZeroes(0) == 0);
}