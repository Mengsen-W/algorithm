/*
 * @Date: 2021-07-08 08:50:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-08 08:58:00
 */

#include <cassert>
#include <vector>
using namespace std;

int numSubarraysWithSum(vector<int>& nums, int goal) {
  int n = nums.size();
  int left1 = 0, left2 = 0, right = 0;
  int sum1 = 0, sum2 = 0;
  int ret = 0;
  while (right < n) {
    sum1 += nums[right];
    while (left1 <= right && sum1 > goal) {
      sum1 -= nums[left1];
      left1++;
    }
    sum2 += nums[right];
    while (left2 <= right && sum2 >= goal) {
      sum2 -= nums[left2];
      left2++;
    }
    ret += left2 - left1;
    right++;
  }
  return ret;
}

int main() {
  {
    vector<int> nums{1, 0, 1, 0, 1};
    int goal = 2;
    assert(numSubarraysWithSum(nums, goal) == 4);
  }
  {
    vector<int> nums(5, 0);
    int goal = 0;
    assert(numSubarraysWithSum(nums, goal) == 15);
  }
}