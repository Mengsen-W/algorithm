/*
 * @Date: 2021-09-23 08:39:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-23 08:48:27
 */

#include <cassert>

class Solution {
 public:
  bool isPowerOfThree(int n) {
    while (n && n % 3 == 0) n /= 3;
    return n == 1;
  }
};

int main() {
  assert(Solution().isPowerOfThree(27));
  assert(!Solution().isPowerOfThree(0));
  assert(Solution().isPowerOfThree(9));
  assert(!Solution().isPowerOfThree(45));
}