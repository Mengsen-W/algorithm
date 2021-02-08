/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-08 17:05:23
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-08 17:54:35
 */

#include <climits>
#include <iostream>
#include <vector>

using namespace std;

int house_robber_linear(vector<int>& nums) {
  int size = nums.size();
  if (size <= 2) return max(nums[0], nums[1]);
  vector<int> dp(size, INT_MIN);
  dp[0] = nums[0];
  dp[1] = nums[1];
  for (int i = 2; i < size; ++i) {
    dp[i] = max(dp[i - 2] + nums[i], dp[i - 1]);
  }
  return dp[size - 1];
}

int house_robber_circle(vector<int>& nums) {
  int size = nums.size();
  if (size <= 2) return max(nums[1], nums[2]);
  vector<int> dp1(size - 1, INT_MIN);
  dp1[0] = nums[0];
  dp1[1] = nums[1];
  for (int i = 2; i < size - 1; ++i) {
    dp1[i] = max(dp1[i - 2] + nums[i], dp1[i - 1]);
  }
  int res1 = dp1[size - 2];

  vector<int> dp2(size - 1, INT_MIN);
  dp2[0] = nums[1];
  dp2[1] = nums[2];
  for (int i = 2; i < size - 1; ++i) {
    dp1[i] = max(dp1[i - 2] + nums[i + 1], dp1[i - 1]);
  }
  int res2 = dp2[size - 2];

  return max(res1, res2);
}

int main() {
  vector<int> nums = {};
  {
    nums = {1, 2, 3, 1};
    cout << house_robber_linear(nums) << endl;
    nums = {2, 7, 9, 3, 1};
    cout << house_robber_linear(nums) << endl;
  }
  {
    nums = {2, 3, 2};
    cout << house_robber_circle(nums) << endl;
  }
  return 0;
}