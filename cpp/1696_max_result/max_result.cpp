/*
 * @Date: 2024-02-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-05
 * @FilePath: /algorithm/cpp/1696_max_result/max_result.cpp
 */

#include <cassert>
#include <deque>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxResult(vector<int>& nums, int k) {
    int n = nums.size();
    vector<int> dp(n);
    dp[0] = nums[0];
    deque<int> queue;
    queue.push_back(0);
    for (int i = 1; i < n; i++) {
      while (!queue.empty() && queue.front() < i - k) {
        queue.pop_front();
      }
      dp[i] = dp[queue.front()] + nums[i];
      while (!queue.empty() && dp[queue.back()] <= dp[i]) {
        queue.pop_back();
      }
      queue.push_back(i);
    }
    return dp[n - 1];
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, -1, -2, 4, -7, 3}, 2, 7},
      {{10, -5, -2, 4, 0, 3}, 3, 17},
      {{1, -5, -20, 4, -1, 3, -6, -3}, 2, 0},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maxResult(nums, k) == ans);
  }
}