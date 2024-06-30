/*
 * @Date: 2021-06-07 08:26:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-07 08:31:00
 */

#include <cassert>
#include <cstring>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int findTargetSumWays(vector<int>& nums, int target) {
    int sum = 0;
    for (int& num : nums) {
      sum += num;
    }
    int diff = sum - target;
    if (diff < 0 || diff % 2 != 0) {
      return 0;
    }
    int neg = diff / 2;
    vector<int> dp(neg + 1);
    dp[0] = 1;
    for (int& num : nums) {
      for (int j = neg; j >= num; j--) {
        dp[j] += dp[j - num];
      }
    }
    return dp[neg];
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 1, 1, 1, 1}, 3, 5},
      {{1}, 1, 1},
  };

  for (auto& [nums, target, ans] : tests) {
    assert(Solution().findTargetSumWays(nums, target) == ans);
  }
}