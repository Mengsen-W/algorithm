/*
 * @Date: 2022-01-31 02:47:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-31 03:04:35
 */

#include <cassert>

class Solution {
 public:
  int numberOfSteps(int num) {
    int ret = 0;
    while (num) {
      ret += (num > 1 ? 1 : 0) + (num & 0x01);
      num >>= 1;
    }
    return ret;
  }
};

class Solution2 {
 public:
  int length(unsigned int num) {
    unsigned int clz = 0;
    if ((num >> 16) == 0) {
      clz += 16;
      num <<= 16;
    }
    if ((num >> 24) == 0) {
      clz += 8;
      num <<= 8;
    }
    if ((num >> 28) == 0) {
      clz += 4;
      num <<= 4;
    }
    if ((num >> 30) == 0) {
      clz += 2;
      num <<= 2;
    }
    if ((num >> 31) == 0) {
      clz += 1;
    }
    return 32 - clz;
  }

  int count(int num) {
    num = (num & 0x55555555) + ((num >> 1) & 0x55555555);
    num = (num & 0x33333333) + ((num >> 2) & 0x33333333);
    num = (num & 0x0F0F0F0F) + ((num >> 4) & 0x0F0F0F0F);
    num = (num & 0x00FF00FF) + ((num >> 8) & 0x00FF00FF);
    num = (num & 0x0000FFFF) + ((num >> 16) & 0x0000FFFF);
    return num;
  }

  int numberOfSteps(int num) {
    return num == 0 ? 0 : length(num) - 1 + count(num);
  }
};

int main() {
  assert(Solution().numberOfSteps(14) == 6);
  assert(Solution().numberOfSteps(8) == 4);
  assert(Solution().numberOfSteps(123) == 12);
  assert(Solution2().numberOfSteps(14) == 6);
  assert(Solution2().numberOfSteps(8) == 4);
  assert(Solution2().numberOfSteps(123) == 12);
}
