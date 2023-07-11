/*
 * @Date: 2023-07-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-11
 * @FilePath: /algorithm/cpp/1911_max_alternating_sum/max_alternating_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  long long maxAlternatingSum(vector<int>& nums) {
    long long even = nums[0], odd = 0;
    for (int i = 1; i < nums.size(); i++) {
      even = max(even, odd + nums[i]);
      odd = max(odd, even - nums[i]);
    }
    return even;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{4, 2, 5, 3, 7}, 7},
      {{5, 6, 7, 8}, 8},
      {{6, 2, 1, 2, 4, 5}, 10},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().maxAlternatingSum(nums) == expected);
  }
}