/*
 * @Date: 2022-04-16 16:10:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-16 16:53:45
 * @FilePath: /algorithm/479_largest_palindrome/largest_palindrome.cpp
 */

#include <cassert>
#include <cmath>

class Solution {
 public:
  int largestPalindrome(int n) {
    if (n == 1) {
      return 9;
    }
    int upper = pow(10, n) - 1;
    for (int left = upper;; --left) {  // 枚举回文数的左半部分
      long p = left;
      for (int x = left; x > 0; x /= 10) {
        p = p * 10 + x % 10;  // 翻转左半部分到其自身末尾，构造回文数 p
      }
      for (long x = upper; x * x >= p; --x) {
        if (p % x == 0) {  // x 是 p 的因子
          return p % 1337;
        }
      }
    }
  }
};

int main() {
  assert(Solution().largestPalindrome(2) == 987);
  assert(Solution().largestPalindrome(1) == 9);
}