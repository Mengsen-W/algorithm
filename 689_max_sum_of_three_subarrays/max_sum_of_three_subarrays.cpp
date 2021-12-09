/*
 * @Date: 2021-12-09 04:47:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-09 04:58:11
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> maxSumOfThreeSubarrays(vector<int> &nums, int k) {
    vector<int> ans;
    int sum1 = 0, maxSum1 = 0, maxSum1Idx = 0;
    int sum2 = 0, maxSum12 = 0, maxSum12Idx1 = 0, maxSum12Idx2 = 0;
    int sum3 = 0, maxTotal = 0;
    int nums_size = nums.size();
    for (int i = k * 2; i < nums_size; ++i) {
      sum1 += nums[i - k * 2];
      sum2 += nums[i - k];
      sum3 += nums[i];
      if (i >= k * 3 - 1) {
        if (sum1 > maxSum1) {
          maxSum1 = sum1;
          maxSum1Idx = i - k * 3 + 1;
        }
        if (maxSum1 + sum2 > maxSum12) {
          maxSum12 = maxSum1 + sum2;
          maxSum12Idx1 = maxSum1Idx;
          maxSum12Idx2 = i - k * 2 + 1;
        }
        if (maxSum12 + sum3 > maxTotal) {
          maxTotal = maxSum12 + sum3;
          ans = {maxSum12Idx1, maxSum12Idx2, i - k + 1};
        }
        sum1 -= nums[i - k * 3 + 1];
        sum2 -= nums[i - k * 2 + 1];
        sum3 -= nums[i - k + 1];
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums = {1, 2, 1, 2, 6, 7, 5, 1};
    int k = 2;
    assert(Solution().maxSumOfThreeSubarrays(nums, k) ==
           vector<int>({0, 3, 5}));
  }
  {
    vector<int> nums = {1, 2, 1, 2, 1, 2, 1, 2, 1};
    int k = 2;
    assert(Solution().maxSumOfThreeSubarrays(nums, k) ==
           vector<int>({0, 2, 4}));
  }
}
