/*
 * @Date: 2021-10-30 01:12:59
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-16
 */

#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> singleNumber(vector<int>& nums) {
    int xorsum = 0;
    for (int num : nums) xorsum ^= num;

    // 防止溢出
    int lsb = (xorsum == INT_MIN ? xorsum : xorsum & (-xorsum));
    int type1 = 0, type2 = 0;
    for (int num : nums)
      if (num & lsb)
        type1 ^= num;
      else
        type2 ^= num;

    return {type1, type2};
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{1, 2, 1, 3, 2, 5}, {3, 5}},
      {{-1, 0}, {-1, 0}},
      {{0, 1}, {1, 0}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().singleNumber(nums) == ans);
  }

  return 0;
}