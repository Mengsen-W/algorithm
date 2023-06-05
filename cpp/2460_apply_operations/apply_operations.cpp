/*
 * @Date: 2023-06-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-05
 * @FilePath: /algorithm/cpp/2460_apply_operations/apply_operations.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> applyOperations(vector<int>& nums) {
    int n = nums.size();
    for (int i = 0, j = 0; i < n; i++) {
      if (i + 1 < n && nums[i] == nums[i + 1]) {
        nums[i] *= 2;
        nums[i + 1] = 0;
      }
      if (nums[i] != 0) {
        swap(nums[i], nums[j]);
        j++;
      }
    }
    return nums;
  }
};

int main() {
  {
    vector<int> nums{1, 2, 2, 1, 1, 0};
    vector<int> ans{1, 4, 2, 0, 0, 0};
    assert(Solution().applyOperations(nums) == ans);
  }

  {
    vector<int> nums{0, 1};
    vector<int> ans{1, 0};
    assert(Solution().applyOperations(nums) == ans);
  }
}
