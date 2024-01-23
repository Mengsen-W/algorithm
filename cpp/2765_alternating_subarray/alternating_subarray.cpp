/*
 * @Date: 2024-01-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-23
 * @FilePath: /algorithm/cpp/2765_alternating_subarray/alternating_subarray.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int alternatingSubarray(vector<int>& nums) {
    int res = -1;
    int n = nums.size();
    int firstIndex = 0;
    for (int i = 1; i < n; i++) {
      int length = i - firstIndex + 1;
      if (nums[i] - nums[firstIndex] == (length - 1) % 2) {
        res = max(res, length);
      } else {
        if (nums[i] - nums[i - 1] == 1) {
          firstIndex = i - 1;
          res = max(res, 2);
        } else {
          firstIndex = i;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 3, 4, 3, 4}, 4},
      {{4, 5, 6}, 2},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().alternatingSubarray(nums) == ans);
  }
}
