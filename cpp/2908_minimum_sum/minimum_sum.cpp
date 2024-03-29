/*
 * @Date: 2024-03-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-29
 * @FilePath: /algorithm/cpp/2908_minimum_sum/minimum_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumSum(vector<int>& nums) {
    int n = nums.size(), res = 1000, mn = 1000;
    vector<int> left(n);
    for (int i = 1; i < n; i++) {
      left[i] = mn = min(nums[i - 1], mn);
    }

    int right = nums[n - 1];
    for (int i = n - 2; i > 0; i--) {
      if (left[i] < nums[i] && nums[i] > right) {
        res = min(res, left[i] + nums[i] + right);
      }
      right = min(right, nums[i]);
    }

    return res < 1000 ? res : -1;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{8, 6, 1, 5, 3}, 9},
      {{5, 4, 8, 7, 10, 2}, 13},
      {{6, 5, 4, 3, 4, 5}, -1},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minimumSum(nums) == ans);
  }
}