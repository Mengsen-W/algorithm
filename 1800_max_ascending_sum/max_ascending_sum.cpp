/*
 * @Date: 2022-10-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-07
 * @FilePath: /algorithm/1800_max_ascending_sum/max_ascending_sum.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxAscendingSum(vector<int>& nums) {
    int res = 0;
    int l = 0;
    while (l < nums.size()) {
      int cursum = nums[l++];
      while (l < nums.size() && nums[l] > nums[l - 1]) {
        cursum += nums[l++];
      }
      res = max(res, cursum);
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{10, 20, 30, 5, 10, 50};
    assert(Solution().maxAscendingSum(nums) == 65);
  }

  {
    vector<int> nums{10, 20, 30, 40, 50};
    assert(Solution().maxAscendingSum(nums) == 150);
  }

  {
    vector<int> nums{12, 17, 15, 13, 10, 11, 12};
    assert(Solution().maxAscendingSum(nums) == 33);
  }

  {
    vector<int> nums{100, 10, 1};
    assert(Solution().maxAscendingSum(nums) == 100);
  }
}