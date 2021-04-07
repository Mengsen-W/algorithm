/*
 * @Date: 2021-04-07 10:30:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-07 10:32:57
 */

#include <cassert>
#include <vector>
using namespace std;

bool search(vector<int>& nums, int target) {
  int l = 0, r = nums.size() - 1;
  while (l <= r) {
    //处理重复数字
    while (l < r && nums[l] == nums[l + 1]) ++l;
    while (l < r && nums[r] == nums[r - 1]) --r;
    int mid = l + (r - l) / 2;
    if (nums[mid] == target) return true;
    //左半部分有序
    if (nums[mid] >= nums[l]) {
      if (target < nums[mid] && target >= nums[l])
        r = mid - 1;  // target落在左半边
      else
        l = mid + 1;
    } else {  //右半部分有序
      if (target > nums[mid] && target <= nums[r])
        l = mid + 1;  // target落在右半边
      else
        r = mid - 1;
    }
  }
  return false;
}

int main() {
  vector<int> nums{2, 5, 6, 0, 0, 1, 2};
  assert(search(nums, 0));
  nums = {2, 5, 6, 0, 0, 1, 2};
  assert(!search(nums, 3));
  return 0;
}
