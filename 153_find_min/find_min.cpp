/*
 * @Date: 2021-04-08 10:15:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-08 10:32:49
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
    } else {
      low = pivot + 1;
    }
  }
  return nums[low];
}

int main() {
  vector<int> nums{3, 4, 5, 1, 2};
  assert(find_min(nums) == 1);
  nums = {4, 5, 6, 7, 0, 1, 2};
  assert(find_min(nums) == 0);
  nums = {11, 13, 15, 17};
  assert(find_min(nums) == 11);
}