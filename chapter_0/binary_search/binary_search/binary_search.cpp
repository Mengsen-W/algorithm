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

int left_bound(const vector<int>& nums, const int& target) {
  int left = 0;
  int right = nums.size() - 1;

  while (left <= right) {
    int mid = left + (right - left) / 2;
    if (nums[mid] == target)
      right = mid - 1;
    else if (nums[mid] < target)
      left = mid + 1;
    else if (nums[mid] > target)
      right = mid - 1;
  }
  if (left >= nums.size() || nums[left] != target) return -1;
  return left;
}

int right_bound(const vector<int>& nums, const int& target) {
  int left = 0;
  int right = nums.size() - 1;

  while (left <= right) {
    int mid = left + (right - left) / 2;
    if (nums[mid] == target)
      left = mid + 1;
    else if (nums[mid] < target)
      left = mid + 1;
    else if (nums[mid] > target)
      right = mid - 1;
  }
  if (right < 0 || nums[right] != target) return -1;
  return right;
}

int main() {
  vector<int> nums = {1, 2, 3, 4, 5};
  int target = 3;
  std::cout << binary_search(nums, target) << std::endl;

  nums = {2, 2, 2, 2, 2};
  target = 2;
  std::cout << left_bound(nums, target) << std::endl;
  std::cout << right_bound(nums, target) << std::endl;

  nums = {1, 2, 2, 2, 3};
  std::cout << left_bound(nums, target) << std::endl;
  std::cout << right_bound(nums, target) << std::endl;

  target = 4;
  std::cout << left_bound(nums, target) << std::endl;
  std::cout << right_bound(nums, target) << std::endl;

  return 0;
}