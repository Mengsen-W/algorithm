/*
 * @Date: 2022-11-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-27
 * @FilePath: /algorithm/1752_check/check.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  bool check(vector<int>& nums) {
    int n = nums.size(), x = 0;
    for (int i = 1; i < n; ++i) {
      if (nums[i] < nums[i - 1]) {
        x = i;
        break;
      }
    }
    if (x == 0) {
      return true;
    }
    for (int i = x + 1; i < n; ++i) {
      if (nums[i] < nums[i - 1]) {
        return false;
      }
    }
    return nums[0] >= nums[n - 1];
  }
};

int main() {
  {
    vector<int> nums{3, 4, 5, 1, 2};
    assert(Solution().check(nums));
  }

  {
    vector<int> nums{2, 1, 3, 4};
    assert(!Solution().check(nums));
  }

  {
    vector<int> nums{1, 2, 3};
    assert(Solution().check(nums));
  }
}