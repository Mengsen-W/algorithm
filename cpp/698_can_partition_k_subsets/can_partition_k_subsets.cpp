/*
 * @Date: 2022-09-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-20
 * @FilePath: /algorithm/698_can_partition_k_subsets/can_partition_k_subsets.cpp
 */

#include <assert.h>

#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canPartitionKSubsets(vector<int>& nums, int k) {
    int all = accumulate(nums.begin(), nums.end(), 0);
    if (all % k > 0) {
      return false;
    }
    int per = all / k;
    sort(nums.begin(), nums.end());
    if (nums.back() > per) {
      return false;
    }
    int n = nums.size();
    vector<bool> dp(1 << n, false);
    vector<int> curSum(1 << n, 0);
    dp[0] = true;
    for (int i = 0; i < 1 << n; i++) {
      if (!dp[i]) {
        continue;
      }
      for (int j = 0; j < n; j++) {
        if (curSum[i] + nums[j] > per) {
          break;
        }
        if (((i >> j) & 1) == 0) {
          int next = i | (1 << j);
          if (!dp[next]) {
            curSum[next] = (curSum[i] + nums[j]) % per;
            dp[next] = true;
          }
        }
      }
    }
    return dp[(1 << n) - 1];
  }
};

int main() {
  {
    vector<int> nums{4, 3, 2, 3, 5, 2, 1};
    int k = 4;
    assert(Solution().canPartitionKSubsets(nums, k) == true);
  }

  {
    vector<int> nums{1, 2, 3, 4};
    int k = 3;
    assert(Solution().canPartitionKSubsets(nums, k) == false);
  }
}