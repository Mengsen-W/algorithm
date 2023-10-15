/*
 * @Date: 2021-04-30 09:55:30
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-15
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int singleNumber(vector<int>& nums) {
    int a = 0, b = 0;
    for (int num : nums) {
      b = ~a & (b ^ num);
      a = ~b & (a ^ num);
    }
    return b;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 2, 3, 2}, 3},
      {{0, 1, 0, 1, 0, 1, 99}, 99},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().singleNumber(nums) == ans);
  }
}