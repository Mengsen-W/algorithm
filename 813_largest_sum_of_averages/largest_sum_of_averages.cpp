/*
 * @Date: 2022-11-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-28
 * @FilePath: /algorithm/813_largest_sum_of_averages/largest_sum_of_averages.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  double largestSumOfAverages(vector<int>& nums, int k) {
    int n = nums.size();
    vector<double> prefix(n + 1);
    for (int i = 0; i < n; i++) {
      prefix[i + 1] = prefix[i] + nums[i];
    }
    vector<double> dp(n + 1);
    for (int i = 1; i <= n; i++) {
      dp[i] = prefix[i] / i;
    }
    for (int j = 2; j <= k; j++) {
      for (int i = n; i >= j; i--) {
        for (int x = j - 1; x < i; x++) {
          dp[i] = max(dp[i], dp[x] + (prefix[i] - prefix[x]) / (i - x));
        }
      }
    }
    return dp[n];
  }
};

int main() {
  {
    vector<int> nums{9, 1, 2, 3, 9};
    int k = 3;
    double ans = 20.0000000000;
    assert(Solution().largestSumOfAverages(nums, k) == ans);
  }

  {
    vector<int> nums{1, 2, 3, 4, 5, 6, 7};
    int k = 4;
    double ans = 20.50000;
    assert(Solution().largestSumOfAverages(nums, k) == ans);
  }
}