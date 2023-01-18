/*
 * @Date: 2021-05-13 08:33:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-13 08:42:48
 */

#include <cassert>
#include <vector>

using namespace std;

static constexpr int MOD = 1e9 + 7;
int numWays(int steps, int arrLen) {
  int len = min(steps, arrLen - 1) + 1;
  vector<int> dp(len, 0);
  dp[0] = 1;
  for (int step = 1; step <= steps; step++) {
    vector<int> dp_nxt(len);
    dp_nxt[0] = (0LL + dp[0] + dp[1]) % MOD;
    for (int idx = 1; idx < len - 1 && dp_nxt[idx - 1]; idx++)
      dp_nxt[idx] = (0LL + dp[idx - 1] + dp[idx] + dp[idx + 1]) % MOD;
    dp_nxt[len - 1] = (0LL + dp[len - 2] + dp[len - 1]) % MOD;
    dp = move(dp_nxt);
  }
  return dp[0];
}

int main() {
  assert(numWays(3, 2) == 4);
  assert(numWays(2, 4) == 2);
}