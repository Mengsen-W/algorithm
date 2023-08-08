/*
 * @Date: 2023-08-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-08
 * @FilePath: /algorithm/cpp/1749_max_absolute_sum/max_absolute_sumc.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxAbsoluteSum(vector<int>& nums) {
    int positiveMax = 0, negativeMin = 0;
    int positiveSum = 0, negativeSum = 0;
    for (int num : nums) {
      positiveSum += num;
      positiveMax = max(positiveMax, positiveSum);
      positiveSum = max(0, positiveSum);
      negativeSum += num;
      negativeMin = min(negativeMin, negativeSum);
      negativeSum = min(0, negativeSum);
    }
    return max(positiveMax, -negativeMin);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, -3, 2, 3, -4}, 5},
      {{2, -5, 1, -4, 3, -2}, 8},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxAbsoluteSum(nums) == ans);
  }
}