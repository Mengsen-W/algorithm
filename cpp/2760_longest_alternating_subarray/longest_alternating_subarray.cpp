/*
 * @Date: 2023-11-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-16
 * @FilePath: /algorithm/cpp/2760_longest_alternating_subarray/longest_alternating_subarray.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestAlternatingSubarray(vector<int>& nums, int threshold) {
    int res = 0, dp = 0, n = nums.size();
    for (int l = n - 1; l >= 0; l--) {
      if (nums[l] > threshold) {
        dp = 0;
      } else if (l == n - 1 || nums[l] % 2 != nums[l + 1] % 2) {
        dp++;
      } else {
        dp = 1;
      }
      if (nums[l] % 2 == 0) {
        res = max(res, dp);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{3, 2, 5, 4}, 5, 3},
      {{1, 2}, 2, 1},
      {{2, 3, 4, 5}, 4, 3},
  };

  for (auto& [nums, threshold, ans] : tests) {
    assert(Solution().longestAlternatingSubarray(nums, threshold) == ans);
  }
}