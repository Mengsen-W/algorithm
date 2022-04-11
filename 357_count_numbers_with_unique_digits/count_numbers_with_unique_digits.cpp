/*
 * @Date: 2022-04-11 10:28:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-11 10:34:30
 * @FilePath: /algorithm/357_count_numbers_with_unique_digits/count_numbers_with_unique_digits.cpp
 */

#include <cassert>

class Solution {
 public:
  int countNumbersWithUniqueDigits(int n) {
    if (n == 0) {
      return 1;
    }
    if (n == 1) {
      return 10;
    }
    int ans = 10, cur = 9;
    for (int i = 0; i < n - 1; ++i) {
      cur *= 9 - i;
      ans += cur;
    }
    return ans;
  }
};

int main() {
  assert(Solution().countNumbersWithUniqueDigits(2) == 91);
  assert(Solution().countNumbersWithUniqueDigits(0) == 1);
}