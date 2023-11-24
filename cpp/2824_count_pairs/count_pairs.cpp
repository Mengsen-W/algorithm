/*
 * @Date: 2023-11-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-24
 * @FilePath: /algorithm/cpp/2824_count_pairs/count_pairs.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countPairs(vector<int>& nums, int target) {
    sort(nums.begin(), nums.end());
    int res = 0;
    for (int i = 0, j = nums.size() - 1; i < j; i++) {
      while (i < j && nums[i] + nums[j] >= target) {
        j--;
      }
      res += j - i;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{-1, 1, 2, 3, 1}, 2, 3},
      {{-6, 2, 5, -2, -7, -1, 3}, -2, 10},
  };

  for (auto& [nums, target, ans] : tests) {
    assert(Solution().countPairs(nums, target) == ans);
  }
}