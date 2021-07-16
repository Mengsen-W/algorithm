/*
 * @Date: 2021-07-16 09:44:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-16 10:45:37
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  int binarySearch(vector<int>& nums, int target, bool lower) {
    int left = 0, right = (int)nums.size() - 1, ans = (int)nums.size();
    while (left <= right) {
      int mid = (left + right) / 2;
      if (nums[mid] > target || (lower && nums[mid] >= target)) {
        right = mid - 1;
        ans = mid;
      } else {
        left = mid + 1;
      }
    }
    return ans;
  }

  int search(vector<int>& nums, int target) {
    int leftIdx = binarySearch(nums, target, true);
    int rightIdx = binarySearch(nums, target, false) - 1;
    if (leftIdx <= rightIdx && rightIdx < nums.size() &&
        nums[leftIdx] == target && nums[rightIdx] == target) {
      return rightIdx - leftIdx + 1;
    }
    return 0;
  }
};

int main() {
  {
    vector<int> nums{5, 7, 7, 8, 8, 10};
    int target = 8;
    assert(Solution().search(nums, target) == 2);
  }
  {
    vector<int> nums{5, 7, 7, 8, 8, 10};
    int target = 6;
    assert(Solution().search(nums, target) == 0);
  }
}