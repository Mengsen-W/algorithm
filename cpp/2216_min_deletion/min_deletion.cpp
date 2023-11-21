/*
 * @Date: 2023-11-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-21
 * @FilePath: /algorithm/cpp/2216_min_deletion/min_deletion.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minDeletion(vector<int>& nums) {
    int n = nums.size();
    int ans = 0;
    bool check = true;
    for (int i = 0; i + 1 < n; ++i) {
      if (nums[i] == nums[i + 1] && check) {
        ++ans;
      } else {
        check = !check;
      }
    }
    if ((n - ans) % 2 != 0) {
      ++ans;
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 1, 2, 3, 5}, 1},
      {{1, 1, 2, 2, 3, 3}, 2},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minDeletion(nums) == ans);
  }
}
