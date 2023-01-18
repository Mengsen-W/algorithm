/*
 * @Date: 2021-04-15 09:17:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-15 09:38:18
 */

#include <cassert>
#include <functional>
#include <vector>
using namespace std;

int robhelp(vector<int>& nums, int start, int end);
int rob(vector<int>& nums);

int rob(vector<int>& nums) {
  int s = nums.size();
  if (s == 1) return nums[0];
  function<int(vector<int>&, int, int)> rob_help =
      [](vector<int>& nums, int start, int end) -> int {
    if (start > end) return 0;
    int dp_i = 0, dp_1 = 0, dp_2 = 0;
    for (int i = start; i <= end; ++i) {
      dp_i = max(dp_2 + nums[i], dp_1);
      // update
      dp_2 = dp_1;
      dp_1 = dp_i;
    }

    return dp_i;
  };
  return max(rob_help(nums, 0, nums.size() - 2),
             rob_help(nums, 1, nums.size() - 1));
}

int robhelp(vector<int>& nums, int start, int end) {
  if (start > end) return 0;
  int dp_i = 0, dp_1 = 0, dp_2 = 0;
  for (int i = start; i <= end; ++i) {
    dp_i = max(dp_2 + nums[i], dp_1);
    // update
    dp_2 = dp_1;
    dp_1 = dp_i;
  }

  return dp_i;
}

// 区别是首位的房子不能同时被抢
// 三种可能 [start, end) (start, end] (start, end)
// 前两种比最后一种多一个选择，至少和最后一种相等
// 所以这里我们选择前两种，但是可能会有重复计算

int main() {
  {
    vector<int> nums{2, 3, 2};
    assert(rob(nums) == 3);
  }
  {
    vector<int> nums{1, 2, 3, 1};
    assert(rob(nums) == 4);
  }
  {
    vector<int> nums{0};
    assert(rob(nums) == 0);
  }
}