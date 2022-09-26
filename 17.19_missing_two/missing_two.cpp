/*
 * @Date: 2022-09-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-26
 * @FilePath: /algorithm/17.19_missing_two/missing_two.cpp
 */

#include <assert.h>

#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> missingTwo(vector<int>& nums) {
    int xorsum = 0;
    int n = nums.size() + 2;
    for (int num : nums) {
      xorsum ^= num;
    }
    for (int i = 1; i <= n; i++) {
      xorsum ^= i;
    }
    // 防止溢出
    int lsb = (xorsum == INT_MIN ? xorsum : xorsum & (-xorsum));
    int type1 = 0, type2 = 0;
    for (int num : nums) {
      if (num & lsb) {
        type1 ^= num;
      } else {
        type2 ^= num;
      }
    }
    for (int i = 1; i <= n; i++) {
      if (i & lsb) {
        type1 ^= i;
      } else {
        type2 ^= i;
      }
    }
    return {type1, type2};
  }
};

int main() {
  {
    vector<int> nums{1};
    vector<int> ans{3, 2};
    assert(Solution().missingTwo(nums) == ans);
  }

  {
    vector<int> nums{2, 3};
    vector<int> ans{1, 4};
    assert(Solution().missingTwo(nums) == ans);
  }
}