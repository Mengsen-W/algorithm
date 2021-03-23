/*
 * @Date: 2021-03-22 08:13:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-22 08:21:01
 */

#include <cassert>

int hamming_weight(unsigned long long n) {
  return (n > 0) ? 1 + hamming_weight(n & (n - 1)) : 0;
}

int main() {
  assert(hamming_weight(0b00000000000000000000000000001011) == 3);
  assert(hamming_weight(0b00000000000000000000000010000000) == 1);
  assert(hamming_weight(0b11111111111111111111111111111101) == 31);
  return 0;
}