/*
 * @Date: 2021-05-27 09:40:52
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-27 09:50:33
 */

#include <cassert>

int hammingDistance(int x, int y) { return __builtin_popcount(x ^ y); }

int hammingDistance_move_bits(int x, int y) {
  int s = x ^ y, ret = 0;
  while (s) {
    ret += s & 1;
    s >>= 1;
  }
  return ret;
}

int hammingDistance_BK(int x, int y) {
  int s = x ^ y, ret = 0;
  while (s) {
    s &= s - 1;
    ret++;
  }
  return ret;
}

int main() {
  assert(hammingDistance(1, 4) == 2);
  assert(hammingDistance_move_bits(1, 4) == 2);
  assert(hammingDistance_BK(1, 4) == 2);
  return 0;
}