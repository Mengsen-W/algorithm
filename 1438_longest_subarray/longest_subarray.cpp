/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-21 09:12:55
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-21 09:19:29
 */

#include <cassert>
#include <set>
#include <vector>

using namespace std;

int longest_subarray(vector<int>& nums, int limit) {
  // 排序可重复的Set
  multiset<int> windows;
  int ans = 0;
  // 定义滑动窗口的左端与右端
  int left = 0, right = 0;
  while (right < nums.size()) {
    // 窗口右移
    windows.insert(nums[right]);
    // 如果遇到不满足条件的情况，窗口右移
    while (*windows.rbegin() - *windows.begin() > limit) {
      windows.erase(windows.find(nums[left]));
      left++;
    }
    // 更新结果
    ans = max(ans, right - left + 1);
    right++;
  }
  return ans;
}

int main(void) {
  vector<int> nums{};
  nums = {8, 2, 4, 7};
  assert(longest_subarray(nums, 4) == 2);
  nums = {10, 1, 2, 4, 7, 2};
  assert(longest_subarray(nums, 5) == 4);
  nums = {4, 2, 2, 2, 4, 4, 2, 2};
  assert(longest_subarray(nums, 0) == 3);
}