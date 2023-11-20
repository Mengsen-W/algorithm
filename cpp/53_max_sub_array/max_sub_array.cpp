/*
 * @Date: 2023-11-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-20
 * @FilePath: /algorithm/cpp/53_max_sub_array/max_sub_array.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxSubArray(vector<int>& nums) {
    int pre = 0, maxAns = nums[0];
    for (const auto& x : nums) {
      pre = max(pre + x, x);
      maxAns = max(maxAns, pre);
    }
    return maxAns;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{-2, 1, -3, 4, -1, 2, 1, -5, 4}, 6},
      {{1}, 1},
      {{5, 4, -1, 7, 8}, 23},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maxSubArray(nums) == ans);
  }
}