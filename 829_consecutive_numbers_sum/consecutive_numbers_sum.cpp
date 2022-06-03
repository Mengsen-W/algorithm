/*
 * @Date: 2022-06-03 23:11:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-03 23:14:39
 * @FilePath: /algorithm/829_consecutive_numbers_sum/consecutive_numbers_sum.cpp
 */

#include <cassert>

class Solution {
 public:
  int consecutiveNumbersSum(int n) {
    int ans = 0;
    int bound = 2 * n;
    for (int k = 1; k * (k + 1) <= bound; k++) {
      if (isKConsecutive(n, k)) {
        ans++;
      }
    }
    return ans;
  }

  bool isKConsecutive(int n, int k) {
    if (k % 2 == 1) {
      return n % k == 0;
    } else {
      return n % k != 0 && 2 * n % k == 0;
    }
  }
};

int main() {
  assert(Solution().consecutiveNumbersSum(5) == 2);
  assert(Solution().consecutiveNumbersSum(9) == 3);
  assert(Solution().consecutiveNumbersSum(15) == 4);
}