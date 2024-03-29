/*
 * @Date: 2023-06-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-28
 * @FilePath: /algorithm/cpp/1681_minimum_incompatibility/minimum_incompatibility.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumIncompatibility(vector<int>& nums, int k) {
    int n = nums.size();
    vector<int> dp(1 << n, INT_MAX);
    dp[0] = 0;
    int group = n / k;
    unordered_map<int, int> values;

    for (int mask = 1; mask < (1 << n); mask++) {
      if (__builtin_popcount(mask) != group) {
        continue;
      }
      int mn = 20, mx = 0;
      unordered_set<int> cur;
      for (int i = 0; i < n; i++) {
        if (mask & (1 << i)) {
          if (cur.count(nums[i]) > 0) {
            break;
          }
          cur.insert(nums[i]);
          mn = min(mn, nums[i]);
          mx = max(mx, nums[i]);
        }
      }
      if (cur.size() == group) {
        values[mask] = mx - mn;
      }
    }

    for (int mask = 0; mask < (1 << n); mask++) {
      if (dp[mask] == INT_MAX) {
        continue;
      }
      unordered_map<int, int> seen;
      for (int i = 0; i < n; i++) {
        if ((mask & (1 << i)) == 0) {
          seen[nums[i]] = i;
        }
      }
      if (seen.size() < group) {
        continue;
      }
      int sub = 0;
      for (auto& pair : seen) {
        sub |= (1 << pair.second);
      }
      int nxt = sub;
      while (nxt > 0) {
        if (values.count(nxt) > 0) {
          dp[mask | nxt] = min(dp[mask | nxt], dp[mask] + values[nxt]);
        }
        nxt = (nxt - 1) & sub;
      }
    }

    return (dp[(1 << n) - 1] < INT_MAX) ? dp[(1 << n) - 1] : -1;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> testMap{
      {vector<int>{1, 2, 1, 4}, 2, 4},
      {vector<int>{6, 3, 8, 1, 3, 1, 2, 2}, 4, 6},
      {vector<int>{5, 3, 3, 6, 3, 3}, 3, -1},
  };

  for (auto& [nums, k, ans] : testMap) {
    assert(Solution().minimumIncompatibility(nums, k) == ans);
  }
}