/*
 * @Date: 2021-04-06 01:00:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-06 01:06:43
 */

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

int remove_duplicates(vector<int>& nums) {
  int n = nums.size();
  if (n <= 2) {
    return n;
  }
  int slow = 2, fast = 2;
  while (fast < n) {
    if (nums[slow - 2] != nums[fast]) {
      nums[slow] = nums[fast];
      ++slow;
    }
    ++fast;
  }
  return slow;
}

int main() {
  vector<int> nums = {1, 1, 1, 2, 2, 3};
  assert(5 == remove_duplicates(nums));
  for (int i : nums) cout << i << ", ";
  cout << endl;
  nums = {0, 0, 1, 1, 1, 1, 2, 3, 3};
  assert(7 == remove_duplicates(nums));
  for (int i : nums) cout << i << ", ";
  return 0;
}