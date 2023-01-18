/*
 * @Date: 2022-02-26 01:33:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-26 01:39:56
 * @FilePath: /algorithm/2016_maximum_difference/maximum_difference.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumDifference(vector<int> nums) {
    int n = nums.size();
    int ans = -1, premin = nums[0];
    for (int i = 1; i < n; ++i) {
      if (nums[i] > premin) {
        ans = max(ans, nums[i] - premin);
      } else {
        premin = nums[i];
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().maximumDifference({7, 1, 5, 4}) == 4);
  assert(Solution().maximumDifference({9, 4, 3, 2}) == -1);
  assert(Solution().maximumDifference({1, 5, 2, 10}) == 9);
}