/*
 * @Date: 2023-02-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-27
 * @FilePath: /algorithm/cpp/1144_moves_to_make_zigzag/moves_to_make_zigzag.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int help(vector<int>& nums, int pos) {
    int res = 0;
    for (int i = pos; i < nums.size(); i += 2) {
      int a = 0;
      if (i - 1 >= 0) {
        a = max(a, nums[i] - nums[i - 1] + 1);
      }
      if (i + 1 < nums.size()) {
        a = max(a, nums[i] - nums[i + 1] + 1);
      }
      res += a;
    }
    return res;
  }

  int movesToMakeZigzag(vector<int>& nums) { return min(help(nums, 0), help(nums, 1)); }
};

int main() {
  {
    vector<int> nums{1, 2, 3};
    int ans = 2;
    assert(Solution().movesToMakeZigzag(nums) == ans);
  }

  {
    vector<int> nums{9, 6, 1, 6, 2};
    int ans = 4;
    assert(Solution().movesToMakeZigzag(nums) == ans);
  }
}