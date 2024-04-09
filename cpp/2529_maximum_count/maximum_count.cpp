/*
 * @Date: 2024-04-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-09
 * @FilePath: /algorithm/cpp/2529_maximum_count/maximum_count.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int lowerBound(vector<int>& nums, int val) {
    int l = 0, r = nums.size();
    while (l < r) {
      int m = (l + r) / 2;
      if (nums[m] >= val) {
        r = m;
      } else {
        l = m + 1;
      }
    }
    return l;
  }

  int maximumCount(vector<int>& nums) {
    int pos1 = lowerBound(nums, 0);
    int pos2 = lowerBound(nums, 1);
    return max(pos1, (int)nums.size() - pos2);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{-2, -1, -1, 1, 2, 3}, 3},
      {{-3, -2, -1, 0, 0, 1, 2}, 3},
      {{5, 20, 66, 1314}, 4},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().maximumCount(nums) == ans);
  }
}