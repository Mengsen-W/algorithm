/*
 * @Date: 2022-03-28 14:59:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-28 15:01:33
 * @FilePath: /algorithm/693_has_alternating_bits/has_alternating_bits.cpp
 */

#include <cassert>

class Solution {
 public:
  bool hasAlternatingBits(int n) {
    long a = n ^ (n >> 1);
    return (a & (a + 1)) == 0;
  }
};

int main() {
  assert(Solution().hasAlternatingBits(5) == true);
  assert(Solution().hasAlternatingBits(7) == false);
  assert(Solution().hasAlternatingBits(11) == false);
}
