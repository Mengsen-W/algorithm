/*
 * @Date: 2023-12-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-22
 * @FilePath: /algorithm/cpp/1671_minimum_mountain_removals/minimum_mountain_removals.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumMountainRemovals(vector<int>& nums) {
    int n = nums.size();
    vector<int> pre = getLISArray(nums);
    vector<int> suf = getLISArray({nums.rbegin(), nums.rend()});
    reverse(suf.begin(), suf.end());

    int ans = 0;
    for (int i = 0; i < n; ++i) {
      if (pre[i] > 1 && suf[i] > 1) {
        ans = max(ans, pre[i] + suf[i] - 1);
      }
    }

    return n - ans;
  }

  vector<int> getLISArray(const vector<int>& nums) {
    int n = nums.size();
    vector<int> dp(n), seq;
    for (int i = 0; i < n; ++i) {
      auto it = lower_bound(seq.begin(), seq.end(), nums[i]);
      if (it == seq.end()) {
        seq.push_back(nums[i]);
        dp[i] = seq.size();
      } else {
        *it = nums[i];
        dp[i] = it - seq.begin() + 1;
      }
    }
    return dp;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3, 1}, 0},
      {{2, 1, 1, 5, 6, 2, 3, 1}, 3},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().minimumMountainRemovals(nums) == ans);
  }
}
