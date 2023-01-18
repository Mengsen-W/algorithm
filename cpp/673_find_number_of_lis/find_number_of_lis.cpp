/*
 * @Date: 2021-09-20 09:17:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-20 09:21:46
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findNumberOfLIS(vector<int> &nums) {
    int n = nums.size(), maxLen = 0, ans = 0;
    vector<int> dp(n), cnt(n);
    for (int i = 0; i < n; ++i) {
      dp[i] = 1;
      cnt[i] = 1;
      for (int j = 0; j < i; ++j) {
        if (nums[i] > nums[j]) {
          if (dp[j] + 1 > dp[i]) {
            dp[i] = dp[j] + 1;
            cnt[i] = cnt[j];  // 重置计数
          } else if (dp[j] + 1 == dp[i]) {
            cnt[i] += cnt[j];
          }
        }
      }
      if (dp[i] > maxLen) {
        maxLen = dp[i];
        ans = cnt[i];  // 重置计数
      } else if (dp[i] == maxLen) {
        ans += cnt[i];
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{1, 3, 5, 4, 7};
    assert(Solution().findNumberOfLIS(nums) == 2);
  }
  {
    vector<int> nums{2, 2, 2, 2, 2};
    assert(Solution().findNumberOfLIS(nums) == 5);
  }
}