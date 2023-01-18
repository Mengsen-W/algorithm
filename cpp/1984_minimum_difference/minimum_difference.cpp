/*
 * @Date: 2022-02-11 00:13:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-11 00:26:23
 * @FilePath: /algorithm/1984_minimum_difference/minimum_difference.cpp
 */

#include <algorithm>
#include <cassert>
#include <climits>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumDifference(vector<int> nums, int k) {
    int n = nums.size();
    sort(nums.begin(), nums.end());
    int ans = INT_MAX;
    for (int i = 0; i + k - 1 < n; ++i) {
      ans = min(ans, nums[i + k - 1] - nums[i]);
    }
    return ans;
  }
};

int main() {
  assert(Solution().minimumDifference({90}, 1) == 0);
  assert(Solution().minimumDifference({9, 4, 7, 1}, 2) == 2);
}