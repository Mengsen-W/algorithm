/*
 * @Date: 2021-04-18 09:49:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-18 10:05:14
 */

#include <iostream>
#include <vector>
using namespace std;

int remove_duplicates(vector<int>& nums) {
  if (nums.size() < 2) return nums.size();
  int j = 0;
  for (int i = 1; i < nums.size(); i++)
    // next = this -> ++next
    // next != this -> this->next = next
    if (nums[j] != nums[i]) nums[++j] = nums[i];
  nums.resize(j + 1);
  return ++j;
}

int main() {
  {
    vector<int> nums{1, 1, 2};
    int len = remove_duplicates(nums);
    for (int i = 0; i < len; ++i) cout << i << ',';
    cout << endl;
  }
  {
    vector<int> nums{0, 0, 1, 1, 1, 2, 2, 3, 3, 4};
    int len = remove_duplicates(nums);
    for (int i = 0; i < len; ++i) cout << i << ',';
    cout << endl;
  }
}