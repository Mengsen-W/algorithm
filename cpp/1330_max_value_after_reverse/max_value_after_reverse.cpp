/*
 * @Date: 2023-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-12
 * @FilePath: /algorithm/cpp/1330_max_value_after_reverse/max_value_after_reverse.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxValueAfterReverse(vector<int>& nums) {
    int value = 0, n = nums.size();
    for (int i = 0; i < n - 1; i++) {
      value += abs(nums[i] - nums[i + 1]);
    }
    int mx1 = 0;
    for (int i = 1; i < n - 1; i++) {
      mx1 = max(mx1, abs(nums[0] - nums[i + 1]) - abs(nums[i] - nums[i + 1]));
      mx1 = max(mx1, abs(nums[n - 1] - nums[i - 1]) - abs(nums[i] - nums[i - 1]));
    }
    int mx2 = INT_MIN, mn2 = INT_MAX;
    for (int i = 0; i < n - 1; i++) {
      int x = nums[i], y = nums[i + 1];
      mx2 = max(mx2, min(x, y));
      mn2 = min(mn2, max(x, y));
    }
    return value + max(mx1, 2 * (mx2 - mn2));
  }
};

int main() {
  {
    vector<int> nums{2, 3, 1, 5, 4};
    int ans = 10;
    assert(Solution().maxValueAfterReverse(nums) == ans);
  }

  {
    vector<int> nums{2, 4, 9, 24, 2, 1, 10};
    int ans = 68;
    assert(Solution().maxValueAfterReverse(nums) == ans);
  }
}