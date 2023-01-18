/*
 * @Date: 2022-10-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-24
 * @FilePath: /algorithm/915_partition_disjoint/partition_disjoint.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int partitionDisjoint(vector<int>& nums) {
    int n = nums.size();
    int leftMax = nums[0], leftPos = 0, curMax = nums[0];
    for (int i = 1; i < n - 1; i++) {
      curMax = max(curMax, nums[i]);
      if (nums[i] < leftMax) {
        leftMax = curMax;
        leftPos = i;
      }
    }
    return leftPos + 1;
  }
};

int main() {
  {
    vector<int> nums{5, 0, 3, 8, 6};
    int ans = 3;
    assert(Solution().partitionDisjoint(nums) == ans);
  }

  {
    vector<int> nums{1, 1, 1, 0, 6, 12};
    int ans = 4;
    assert(Solution().partitionDisjoint(nums) == ans);
  }
}
