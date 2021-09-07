/*
 * @Date: 2021-09-07 17:26:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-07 17:35:15
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int search(vector<int>& nums, int target) {
    int low = 0, high = nums.size() - 1;
    while (low <= high) {
      int mid = (high - low) / 2 + low;
      int num = nums[mid];
      if (num == target) {
        return mid;
      } else if (num > target) {
        high = mid - 1;
      } else {
        low = mid + 1;
      }
    }
    return -1;
  }
};

int main() {
  {
    vector<int> nums{-1, 0, 3, 5, 9, 12};
    int target = 9;
    int ans = 4;
    assert(Solution().search(nums, target) == ans);
  }
  {
    vector<int> nums{-1, 0, 3, 5, 9, 12};
    int target = 2;
    int ans = -1;
    assert(Solution().search(nums, target) == ans);
  }
}