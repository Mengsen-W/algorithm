/*
 * @Date: 2022-12-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-11
 * @FilePath: /algorithm/1827_min_operations/min_operations.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int>& nums) {
    int pre = nums[0] - 1, res = 0;
    for (int num : nums) {
      pre = max(pre + 1, num);
      res += pre - num;
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{1, 1, 1};
    int ans = 3;
    assert(Solution().minOperations(nums) == ans);
  }
  {
    vector<int> nums{1,5,2,4,1};
    int ans = 14;
    assert(Solution().minOperations(nums) == ans);
  }
  {
    vector<int> nums{8};
    int ans = 0;
    assert(Solution().minOperations(nums) == ans);
  }
}