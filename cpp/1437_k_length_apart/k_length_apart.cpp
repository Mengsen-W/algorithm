/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 23:36:03
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 23:38:32
 */

#include <iostream>
#include <vector>

using namespace std;

bool k_length_apart(vector<int>& nums, int k) {
  int n = nums.size();
  int pre = -1;
  for (int i = 0; i < n; ++i) {
    if (nums[i] == 1) {
      if (pre != -1 && i - pre - 1 < k) return false;
      pre = i;
    }
  }
  return true;
}

int main(void) {
  vector<int> nums;
  nums = {1, 0, 0, 0, 1, 0, 0, 1};
  cout << k_length_apart(nums, 2) << endl;
  nums = {1, 0, 0, 1, 0, 1};
  cout << k_length_apart(nums, 2) << endl;
  nums = {1, 1, 1, 1, 1};
  cout << k_length_apart(nums, 0) << endl;
  nums = {0, 1, 0, 1};
  cout << k_length_apart(nums, 1) << endl;
}