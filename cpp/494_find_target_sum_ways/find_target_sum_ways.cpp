/*
 * @Date: 2021-06-07 08:26:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-07 08:31:00
 */

#include <cassert>
#include <cstring>
#include <vector>
using namespace std;

int findTargetSumWays(vector<int> &nums, int S) {
  long sum = 0;
  for (const int &it : nums) sum += it;
  if ((S + sum) % 2 == 1 || S > sum) return 0;
  S = (S + sum) / 2;
  int *dp = new int[S + 1];
  memset(dp, 0, (S + 1) * sizeof(int));
  dp[0] = 1;
  for (const int &it : nums) {
    for (int j = S; j >= it; j--) dp[j] += dp[j - it];
  }
  int ans = dp[S];
  delete[] dp;
  return ans;
}

int main() {
  {
    vector<int> nums{1, 1, 1, 1, 1};
    int target = 3;
    int ans = 5;
    assert(findTargetSumWays(nums, target) == ans);
  }
  {
    vector<int> nums{1};
    int target = 1;
    int ans = 1;
    assert(findTargetSumWays(nums, target) == ans);
  }
}