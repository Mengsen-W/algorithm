/*
 * @Date: 2021-08-28 14:50:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-28 15:03:37
 * @FilePath: /algorithm/1480_running_sum/running_sum.cpp
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> runningSum(vector<int>& nums) {
    int n = nums.size();
    for (int i = 1; i < n; i++) {
      nums[i] += nums[i - 1];
    }
    return nums;
  }
};

int main() {
  {
    vector<int> nums{1, 2, 3, 4};
    vector<int> ans{1, 3, 6, 10};
    assert(Solution{}.runningSum(nums) == ans);
  }
  {
    vector<int> nums{1, 1, 1, 1, 1};
    vector<int> ans{1, 2, 3, 4, 5};
    assert(Solution{}.runningSum(nums) == ans);
  }
  {
    vector<int> nums{3, 1, 2, 10, 1};
    vector<int> ans{3, 4, 6, 16, 17};
    assert(Solution{}.runningSum(nums) == ans);
  }
}