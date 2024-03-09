/*
 * @Date: 2024-03-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-09
 * @FilePath: /algorithm/cpp/2386_k_sum/k_sum.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long kSum(vector<int> &nums, int k) {
    int n = nums.size();
    long long total = 0;
    for (int &x : nums) {
      if (x >= 0) {
        total += x;
      } else {
        x = -x;
      }
    }
    sort(nums.begin(), nums.end());

    long long ret = 0;
    priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> pq;
    pq.push({nums[0], 0});
    for (int j = 2; j <= k; j++) {
      auto [t, i] = pq.top();
      pq.pop();
      ret = t;
      if (i == n - 1) {
        continue;
      }
      pq.push({t + nums[i + 1], i + 1});
      pq.push({t - nums[i] + nums[i + 1], i + 1});
    }
    return total - ret;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{2, 4, -2}, 5, 2},
      {{1, -2, 3, 4, -10, 12}, 16, 10},
  };

  for (auto &[nums, k, ans] : tests) {
    assert(Solution().kSum(nums, k) == ans);
  }
}