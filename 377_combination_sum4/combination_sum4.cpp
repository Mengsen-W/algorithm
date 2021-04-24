/*
 * @Date: 2021-04-24 14:33:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-24 14:52:34
 */

#include <cassert>
#include <vector>
using namespace std;

int combination_sum4(vector<int>& nums, int target) {
  vector<int> dp(target + 1);
  dp[0] = 1;
  for (int i = 1; i <= target; i++) {
    for (int& num : nums) {
      // 如果没处理 结果和必然大于INT_MAX 造成超出INT范围
      if (num <= i && dp[i - num] < INT_MAX - dp[i]) {
        dp[i] += dp[i - num];
      }
    }
  }
  return dp[target];
}

int main() {
  {
    vector<int> nums{1, 2, 3};
    assert(combination_sum4(nums, 4) == 7);
  }
  {
    vector<int> nums{9};
    assert(combination_sum4(nums, 3) == 0);
  }
  return 0;
}
