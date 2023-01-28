/*
 * @Date: 2023-01-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-28
 * @FilePath: /algorithm/cpp/1664_ways_to_make_fair/ways_to_make_fair.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int waysToMakeFair(vector<int>& nums) {
    int odd1 = 0, even1 = 0;
    int odd2 = 0, even2 = 0;
    for (int i = 0; i < nums.size(); ++i) {
      if (i & 1) {
        odd2 += nums[i];
      } else {
        even2 += nums[i];
      }
    }
    int res = 0;
    for (int i = 0; i < nums.size(); ++i) {
      if (i & 1) {
        odd2 -= nums[i];
      } else {
        even2 -= nums[i];
      }
      if (odd1 + even2 == odd2 + even1) {
        ++res;
      }
      if (i & 1) {
        odd1 += nums[i];
      } else {
        even1 += nums[i];
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{2, 1, 6, 4};
    int ans = 1;
    assert(Solution().waysToMakeFair(nums) == ans);
  }

  {
    vector<int> nums{1, 1, 1};
    int ans = 3;
    assert(Solution().waysToMakeFair(nums) == ans);
  }

  {
    vector<int> nums{1, 2, 3};
    int ans = 0;
    assert(Solution().waysToMakeFair(nums) == ans);
  }
}