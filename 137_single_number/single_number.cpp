/*
 * @Date: 2021-04-30 09:55:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-30 10:00:10
 */

#include <cassert>
#include <vector>

using namespace std;

int single_number(vector<int>& nums) {
  int a = 0, b = 0;
  for (int num : nums) {
    b = ~a & (b ^ num);
    a = ~b & (a ^ num);
  }
  return b;
}

int main() {
  {
    vector<int> nums{2, 2, 3, 2};
    assert(single_number(nums) == 3);
  }
  {
    vector<int> nums{0, 1, 0, 1, 0, 1, 99};
    assert(single_number(nums) == 99);
  }
}