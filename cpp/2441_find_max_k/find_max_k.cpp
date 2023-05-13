/*
 * @Date: 2023-05-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-13
 * @FilePath: /algorithm/cpp/2441_find_max_k/find_max_k.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findMaxK(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    for (int i = 0, j = nums.size() - 1; i < j; j--) {
      while (i < j && nums[i] < -nums[j]) {
        i++;
      }
      if (nums[i] == -nums[j]) {
        return nums[j];
      }
    }
    return -1;
  }
};

int main() {
  {
    vector<int> nums{-1, 2, -3, 3};
    int ans = 3;
    assert(Solution().findMaxK(nums) == ans);
  }

  {
    vector<int> nums{-1, 10, 6, 7, -7, 1};
    int ans = 7;
    assert(Solution().findMaxK(nums) == ans);
  }

  {
    vector<int> nums{-10, 8, 6, 7, -2, -3};
    int ans = -1;
    assert(Solution().findMaxK(nums) == ans);
  }
}