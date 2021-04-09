/*
 * @Date: 2021-04-09 08:47:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-09 08:59:03
 */

#include <cassert>
#include <vector>

using namespace std;

int find_min(vector<int>& nums) {
  int low = 0;
  int high = nums.size() - 1;
  while (low < high) {
    int pivot = low + (high - low) / 2;
    if (nums[pivot] < nums[high]) {
      high = pivot;
    } else if (nums[pivot] > nums[high]) {
      low = pivot + 1;
    } else {
      high -= 1;
    }
  }
  return nums[low];
}

int main() {
  vector<int> nums{1, 3, 5};
  assert(find_min(nums) == 1);
  nums = {2, 2, 2, 0, 1};
  assert(find_min(nums) == 0);
  return 0;
}
