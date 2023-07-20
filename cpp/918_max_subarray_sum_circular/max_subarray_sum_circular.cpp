/*
 * @Date: 2023-07-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-20
 * @FilePath: /algorithm/cpp/918_max_subarray_sum_circular/max_subarray_sum_circular.cpp
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSubarraySumCircular(vector<int>& nums) {
    int n = nums.size();
    deque<pair<int, int>> q;
    int pre = nums[0], res = nums[0];
    q.push_back({0, pre});
    for (int i = 1; i < 2 * n; i++) {
      while (!q.empty() && q.front().first < i - n) {
        q.pop_front();
      }
      pre += nums[i % n];
      res = max(res, pre - q.front().second);
      while (!q.empty() && q.back().second >= pre) {
        q.pop_back();
      }
      q.push_back({i, pre});
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, -2, 3, -2}, 3},
      {{5, -3, 5}, 10},
      {{3, -2, 2, -3}, 3},
  };
  Solution s;
  for (auto &[nums, ans] : tests) {
    assert(s.maxSubarraySumCircular(nums) == ans);
  }
}
