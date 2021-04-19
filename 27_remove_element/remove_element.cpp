/*
 * @Date: 2021-04-19 08:38:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-19 08:52:30
 */

#include <iostream>
#include <vector>
using namespace std;

void print_vec(const vector<int>& nums, const int& size) {
  for (int i = 0; i < size; i++) cout << nums[i] << ", ";
  cout << endl;
}

int remove_element(vector<int>& nums, int val) {
  if (nums.size() == 0) return 0;
  int j = 0;
  for (int i = 0; i < nums.size(); i++) {
    if (nums[i] != val) {
      nums[j++] = nums[i];
    }
  }
  return j;
}

int main() {
  {
    vector<int> nums{3, 2, 2, 3};
    int size = remove_element(nums, 3);
    print_vec(nums, size);
  }
  {
    vector<int> nums{0, 1, 2, 2, 3, 0, 4, 2};
    int size = remove_element(nums, 2);
    print_vec(nums, size);
  }
}