/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-27 16:40:17
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-27 17:43:00
 */

#include <iostream>
#include <vector>

using namespace std;

int binary_search(const vector<int>& nums, const int& target) {
  int left = 0;
  int right = nums.size() - 1;

  while (left <= right) {
    int mid = left + (right - left) / 2;
    if (nums[mid] == target)
      return mid;
    else if (nums[mid] < target)
      left = mid + 1;
    else if (nums[mid] > target)
      right = mid - 1;
  }
  return -1;
}

int left_bound(const vector<int>& nums, const int& target) {}

int main() {
  vector<int> nums = {1, 2, 3, 4, 5};
  int target = 3;
  cout << binary_search(nums, target);
  return 0;
}