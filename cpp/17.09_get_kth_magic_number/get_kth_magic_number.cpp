/*
 * @Date: 2022-09-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-28
 * @FilePath: /algorithm/17.09_get_kth_magic_number/get_kth_magic_number.cpp
 */

#include <assert.h>

#include <vector>

using namespace std;

class Solution {
 public:
  int getKthMagicNumber(int k) {
    vector<int> dp(k + 1);
    dp[1] = 1;
    int p3 = 1, p5 = 1, p7 = 1;
    for (int i = 2; i <= k; i++) {
      int num3 = dp[p3] * 3, num5 = dp[p5] * 5, num7 = dp[p7] * 7;
      dp[i] = min(min(num3, num5), num7);
      if (dp[i] == num3) {
        p3++;
      }
      if (dp[i] == num5) {
        p5++;
      }
      if (dp[i] == num7) {
        p7++;
      }
    }
    return dp[k];
  }
};

int main() { assert(Solution().getKthMagicNumber(5) == 9); }