/*
 * @Date: 2021-09-26 08:52:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-26 08:53:33
 */

#include <cassert>

class Solution {
 public:
  int getSum(int a, int b) {
    while (b != 0) {
      unsigned int carry = (unsigned int)(a & b) << 1;
      a = a ^ b;
      b = carry;
    }
    return a;
  }
};

int main() {
  assert(Solution().getSum(1, 2) == 3);
  assert(Solution().getSum(2, 3) == 5);
}