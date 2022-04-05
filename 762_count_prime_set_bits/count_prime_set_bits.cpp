/*
 * @Date: 2022-04-05 10:25:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-05 10:30:56
 * @FilePath: /algorithm/762_count_prime_set_bits/count_prime_set_bits.cpp
 */

#include <cassert>

class Solution {
 public:
  int countPrimeSetBits(int left, int right) {
    int ans = 0;
    for (int x = left; x <= right; ++x) {
      if ((1 << __builtin_popcount(x)) & 665772) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().countPrimeSetBits(6, 10) == 4);
  assert(Solution().countPrimeSetBits(10, 15) == 5);
}