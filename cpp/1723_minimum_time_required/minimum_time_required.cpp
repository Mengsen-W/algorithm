/*
 * @Date: 2021-05-08 08:54:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-08 09:13:18
 */

#include <cassert>
#include <vector>
using namespace std;

int minimum_time_required(vector<int>& jobs, int k) {
  int n = jobs.size();
  vector<int> sum(1 << n);
  for (int i = 1; i < (1 << n); i++) {
    int x = __builtin_ctz(i), y = i - (1 << x);
    sum[i] = sum[y] + jobs[x];
  }

  vector<vector<int>> dp(k, vector<int>(1 << n));
  for (int i = 0; i < (1 << n); i++) {
    dp[0][i] = sum[i];
  }

  for (int i = 1; i < k; i++) {
    for (int j = 0; j < (1 << n); j++) {
      int minn = INT_MAX;
      for (int x = j; x; x = (x - 1) & j) {
        minn = min(minn, max(dp[i - 1][j - x], sum[x]));
      }
      dp[i][j] = minn;
    }
  }
  return dp[k - 1][(1 << n) - 1];
}

int main() {
  {
    vector<int> nums{3, 2, 3};
    assert(minimum_time_required(nums, 3) == 3);
  }
  {
    vector<int> nums{1, 2, 4, 7, 8};
    assert(minimum_time_required(nums, 2) == 11);
  }
  return 0;
}