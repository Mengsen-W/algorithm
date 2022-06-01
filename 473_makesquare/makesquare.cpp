/*
 * @Date: 2022-06-01 09:34:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-01 09:36:10
 * @FilePath: /algorithm/473_makesquare/makesquare.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  bool makesquare(vector<int> matchsticks) {
    int totalLen = accumulate(matchsticks.begin(), matchsticks.end(), 0);
    if (totalLen % 4 != 0) {
      return false;
    }
    int len = totalLen / 4, n = matchsticks.size();
    vector<int> dp(1 << n, -1);
    dp[0] = 0;
    for (int s = 1; s < (1 << n); s++) {
      for (int k = 0; k < n; k++) {
        if ((s & (1 << k)) == 0) {
          continue;
        }
        int s1 = s & ~(1 << k);
        if (dp[s1] >= 0 && dp[s1] + matchsticks[k] <= len) {
          dp[s] = (dp[s1] + matchsticks[k]) % len;
          break;
        }
      }
    }
    return dp[(1 << n) - 1] == 0;
  }
};

int main() {
  assert(Solution().makesquare({1, 1, 2, 2, 2}) == true);
  assert(Solution().makesquare({3, 3, 3, 3, 4}) == false);
}