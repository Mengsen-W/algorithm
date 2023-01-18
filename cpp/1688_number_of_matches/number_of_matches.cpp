/*
 * @Date: 2022-01-25 00:27:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-25 00:34:24
 */

#include <cassert>

class Solution {
 public:
  int numberOfMatches1(int n) {
    int ans = 0;
    while (n > 1) {
      if (n % 2 == 0) {
        ans += n / 2;
        n /= 2;
      } else {
        ans += (n - 1) / 2;
        n = (n - 1) / 2 + 1;
      }
    }
    return ans;
  }
  int numberOfMatches2(int n) { return n - 1; }
};

int main() {
  assert(Solution().numberOfMatches1(7) == 6);
  assert(Solution().numberOfMatches1(14) == 13);
  assert(Solution().numberOfMatches2(7) == 6);
  assert(Solution().numberOfMatches2(14) == 13);
}