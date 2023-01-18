/*
 * @Date: 2021-09-11 07:52:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-11 07:57:13
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  int findIntegers(int n) {
    vector<int> dp(31);
    dp[0] = dp[1] = 1;
    for (int i = 2; i < 31; ++i) {
      dp[i] = dp[i - 1] + dp[i - 2];
    }

    int pre = 0, res = 0;
    for (int i = 29; i >= 0; --i) {
      int val = 1 << i;
      if ((n & val) != 0) {
        n -= val;
        res += dp[i + 1];
        if (pre == 1) {
          break;
        }
        pre = 1;
      } else {
        pre = 0;
      }

      if (i == 0) {
        ++res;
      }
    }

    return res;
  }
};

int main(void) {
  assert(Solution().findIntegers(5) == 5);
  return 0;
}