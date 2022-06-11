/*
 * @Date: 2022-06-11 21:44:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-11 21:55:46
 * @FilePath: /algorithm/926_min_flips_mono_incr/min_flips_mono_incr.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int minFlipsMonoIncr(string s) {
    int dp0 = 0, dp1 = 0;
    for (char c : s) {
      int dp0New = dp0, dp1New = min(dp0, dp1);
      if (c == '1') {
        dp0New++;
      } else {
        dp1New++;
      }
      dp0 = dp0New;
      dp1 = dp1New;
    }
    return min(dp0, dp1);
  }
};

int main() {
  assert(Solution().minFlipsMonoIncr("00110") == 1);
  assert(Solution().minFlipsMonoIncr("010110") == 2);
  assert(Solution().minFlipsMonoIncr("00011000") == 2);
}