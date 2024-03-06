/*
 * @Date: 2024-03-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-06
 * @FilePath: /algorithm/cpp/2917_find_k_or/find_k_or.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findKOr(vector<int>& nums, int k) {
    int ans = 0;
    for (int i = 0; i < 31; ++i) {
      int cnt = 0;
      for (int num : nums) {
        if ((num >> i) & 1) {
          ++cnt;
        }
      }
      if (cnt >= k) {
        ans |= 1 << i;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{7, 12, 9, 8, 9, 15}, 4, 9},
      {{2, 12, 1, 11, 4, 5}, 6, 0},
      {{10, 8, 5, 9, 11, 6, 8}, 1, 15},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().findKOr(nums, k) == ans);
  }
}
