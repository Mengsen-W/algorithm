/*
 * @Date: 2021-08-03 15:41:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-03 16:19:03
 */

#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  int findUnsortedSubarray(vector<int>& nums) {
    // return findUnsortedSubarray_sort(nums);
    return findUnsortedSubarray_traverse(nums);
  }

  int findUnsortedSubarray_sort(vector<int>& nums) {
    // 寻找最长的前后端
    if (is_sorted(nums.begin(), nums.end())) return 0;
    vector<int> nums_sorted(nums);
    sort(nums_sorted.begin(), nums_sorted.end());
    int left = 0, right = nums.size() - 1;
    while (nums[left] == nums_sorted[left]) ++left;
    while (nums[right] == nums_sorted[right]) --right;
    return right - left + 1;
  }

  int findUnsortedSubarray_traverse(vector<int>& nums) {
    int n = nums.size();
    int maxn = INT_MIN, right = -1;
    int minn = INT_MAX, left = -1;

    // max 越来越大 min 越来越小
    // 如果发现右侧的比max小，则发生了逆序，更新right
    // 如果发现左侧的比max大，则发生了逆序，更新left

    for (int i = 0; i < n; ++i) {
      if (maxn > nums[i])
        right = i;
      else
        maxn = nums[i];
      if (minn < nums[n - i - 1])
        left = n - i - 1;
      else
        minn = nums[n - i - 1];
    }
    return right == -1 ? 0 : right - left + 1;
  }
};

int main() {
  {
    vector<int> nums{2, 6, 4, 8, 10, 9, 15};
    int ans = 5;
    assert(Solution{}.findUnsortedSubarray_sort(nums) == ans);
    assert(Solution{}.findUnsortedSubarray_traverse(nums) == ans);
  }

  {
    vector<int> nums{1, 2, 3, 4};
    int ans = 0;
    assert(Solution{}.findUnsortedSubarray_sort(nums) == ans);
    assert(Solution{}.findUnsortedSubarray_traverse(nums) == ans);
  }

  {
    vector<int> nums{1};
    int ans = 0;
    assert(Solution{}.findUnsortedSubarray_sort(nums) == ans);
    assert(Solution{}.findUnsortedSubarray_traverse(nums) == ans);
  }

  return 0;
}