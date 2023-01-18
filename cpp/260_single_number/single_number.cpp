/*
 * @Date: 2021-10-30 01:12:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-30 01:23:50
 */

#include <cassert>
#include <climits>
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
  {
    Solution solution;
    vector<int> nums = {1, 2, 1, 3, 2, 5};
    vector<int> result = solution.singleNumber(nums);
    assert(result[0] == 3);
    assert(result[1] == 5);
  }
  {
    Solution solution;
    vector<int> nums = {-1, 0};
    vector<int> result = solution.singleNumber(nums);
    assert(result[0] == -1);
    assert(result[1] == 0);
  }
  {
    Solution solution;
    vector<int> nums = {0, 1};
    vector<int> result = solution.singleNumber(nums);
    assert(result[0] == 1);
    assert(result[1] == 0);
  }
  return 0;
}