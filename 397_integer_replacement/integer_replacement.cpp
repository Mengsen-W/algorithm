/*
 * @Date: 2021-11-19 00:29:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-19 00:38:07
 */

#include <cassert>

class Solution {
 public:
  int integerReplacement(int n) {
    int ans = 0;
    while (n != 1) {
      if (n % 2 == 0) {
        ++ans;
        n /= 2;
      } else if (n % 4 == 1) {
        ans += 2;
        n /= 2;
      } else {
        if (n == 3) {
          ans += 2;
          n = 1;
        } else {
          ans += 2;
          n = n / 2 + 1;
        }
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().integerReplacement(8) == 3);
  assert(Solution().integerReplacement(7) == 4);
  assert(Solution().integerReplacement(4) == 2);
}