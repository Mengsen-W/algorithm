/*
 * @Date: 2024-03-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-14
 * @FilePath: /algorithm/cpp/2789_max_array_value/max_array_value.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maxArrayValue(vector<int>& nums) {
    long long sum = nums.back();
    for (int i = nums.size() - 2; i >= 0; i--) {
      sum = nums[i] <= sum ? nums[i] + sum : nums[i];
    }
    return sum;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{2, 3, 7, 9, 3}, 21},
      {{5, 3, 3}, 11},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxArrayValue(nums) == ans);
  }
}