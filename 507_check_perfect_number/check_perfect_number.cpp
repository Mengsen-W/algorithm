/*
 * @Date: 2021-12-31 01:05:27
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-31 01:17:13
 */

#include <cassert>

class Solution {
 public:
  bool checkPerfectNumber(int num) {
    if (num == 1) {
      return false;
    }

    int sum = 1;
    for (int d = 2; d * d <= num; ++d) {
      if (num % d == 0) {
        sum += d;
        // alert 注意当 d*d=num 时这两个因数相同，此时不能重复计算
        if (d * d < num) {
          sum += num / d;
        }
      }
    }
    return sum == num;
  }
};

int main() {
  assert(Solution().checkPerfectNumber(28) == true);
  assert(Solution().checkPerfectNumber(6) == true);
  assert(Solution().checkPerfectNumber(496) == true);
  assert(Solution().checkPerfectNumber(8128) == true);
  assert(Solution().checkPerfectNumber(2) == false);
}