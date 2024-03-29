/*
 * @Date: 2021-11-30 03:36:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-30 03:37:59
 */

#include <cmath>
#include <cassert>

class Solution {
 public:
  int findNthDigit(int n) {
    int d = 1, count = 9;
    while (n > (long)d * count) {
      n -= d * count;
      d++;
      count *= 10;
    }
    int index = n - 1;
    int start = (int)pow(10, d - 1);
    int num = start + index / d;
    int digitIndex = index % d;
    int digit = (num / (int)(pow(10, d - digitIndex - 1))) % 10;
    return digit;
  }
};


int main() {
  assert(Solution().findNthDigit(3) == 3);
  assert(Solution().findNthDigit(11) == 0);
}