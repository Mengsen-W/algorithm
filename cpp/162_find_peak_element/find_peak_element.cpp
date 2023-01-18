/*
 * @Date: 2021-09-15 08:41:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-15 08:45:12
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  int findPeakElement(vector<int>& nums) {
    int n = nums.size();

    // 辅助函数，输入下标 i，返回一个二元组 (0/1, nums[i])
    // 方便处理 nums[-1] 以及 nums[n] 的边界情况
    auto get = [&](int i) -> pair<int, int> {
      if (i == -1 || i == n) {
        return {0, 0};
      }
      return {1, nums[i]};
    };

    int left = 0, right = n - 1, ans = -1;
    while (left <= right) {
      int mid = (left + right) / 2;
      if (get(mid - 1) < get(mid) && get(mid) > get(mid + 1)) {
        ans = mid;
        break;
      }
      if (get(mid) < get(mid + 1)) {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{1, 2, 3, 1};
    int ans = 2;
    assert(Solution().findPeakElement(nums) == ans);
  }
  {
    vector<int> nums{1, 2, 1, 3, 5, 6, 4};
    int ans = 5;
    assert(Solution().findPeakElement(nums) == ans);
  }
}