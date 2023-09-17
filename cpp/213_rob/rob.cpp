/*
 * @Date: 2023-09-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-17
 * @FilePath: /algorithm/cpp/213_rob/rob.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int robRange(vector<int>& nums, int start, int end) {
    int first = nums[start], second = max(nums[start], nums[start + 1]);
    for (int i = start + 2; i <= end; i++) {
      int temp = second;
      second = max(first + nums[i], second);
      first = temp;
    }
    return second;
  }

  int rob(vector<int>& nums) {
    int length = nums.size();
    if (length == 1) {
      return nums[0];
    } else if (length == 2) {
      return max(nums[0], nums[1]);
    }
    return max(robRange(nums, 0, length - 2), robRange(nums, 1, length - 1));
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 3, 2}, 3},
      {{1, 2, 3, 1}, 4},
      {{1, 2, 3}, 3},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().rob(nums) == ans);
  }
}