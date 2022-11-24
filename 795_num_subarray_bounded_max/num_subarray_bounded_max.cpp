/*
 * @Date: 2022-11-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-24
 * @FilePath: /algorithm/795_num_subarray_bounded_max/num_subarray_bounded_max.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int numSubarrayBoundedMax(vector<int>& nums, int left, int right) {
    int res = 0, last2 = -1, last1 = -1;
    for (int i = 0; i < nums.size(); i++) {
      if (nums[i] >= left && nums[i] <= right) {
        last1 = i;
      } else if (nums[i] > right) {
        last2 = i;
        last1 = -1;
      }
      if (last1 != -1) {
        res += last1 - last2;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{2, 1, 4, 3};
    int left = 2;
    int right = 3;
    int ans = 3;
    assert(Solution().numSubarrayBoundedMax(nums, left, right) == ans);
  }

  {
    vector<int> nums{2,9,2,5,6};
    int left = 2;
    int right = 8;
    int ans = 7;
    assert(Solution().numSubarrayBoundedMax(nums, left, right) == ans);
  }
}