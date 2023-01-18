/*
 * @Date: 2021-06-23 08:45:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-23 08:56:19
 */

#include <cassert>
#include <cstdint>

int hammingWeight(uint32_t n) { return __builtin_popcount(n); }

int main() {
  assert(hammingWeight(0b00000000000000000000000000001011) == 3);
  assert(hammingWeight(0b00000000000000000000000010000000) == 1);
  assert(hammingWeight(0b11111111111111111111111111111101) == 31);
  return 0;
}