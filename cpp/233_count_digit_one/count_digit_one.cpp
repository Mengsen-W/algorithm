/*
 * @Date: 2021-08-13 11:37:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-13 11:57:26
 */

#include <cassert>
#include <cmath>

class Solution {
 public:
  int countDigitOne(int n) {
    int num = n, s = 0;
    long i = 1;
    while (num) {
      //分别计算个、十、百......千位上1出现的次数，再求和。
      if (num % 10 == 0) s = s + (num / 10) * i;
      if (num % 10 == 1) s = s + (num / 10) * i + (n % i) + 1;
      if (num % 10 > 1) s = s + ceil(num / 10.0) * i;
      num = num / 10;
      i = i * 10;
    }
    return s;
  }
};

int main() {
  assert(Solution{}.countDigitOne(13) == 6);
  assert(Solution{}.countDigitOne(0) == 0);
  assert(Solution{}.countDigitOne(1000000000) == 900000001);
}