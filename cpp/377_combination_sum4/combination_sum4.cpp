/*
 * @Date: 2021-04-24 14:33:16
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-22
 */

#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int combinationSum4(vector<int>& nums, int target) {
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
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 2, 3}, 4, 7},
      {{9}, 3, 0},
  };

  for (auto& [nums, target, ans] : tests) {
    assert(Solution().combinationSum4(nums, target) == ans);
  }

  return 0;
}
