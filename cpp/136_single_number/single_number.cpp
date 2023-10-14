/*
 * @Date: 2023-10-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-14
 * @FilePath: /algorithm/cpp/136_single_number/single_number.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int singleNumber(vector<int>& nums) {
    int ret = 0;
    for (auto e : nums) ret ^= e;
    return ret;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 2, 1}, 1},
      {{4, 1, 2, 1, 2}, 4},
      {{1}, 1},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().singleNumber(nums) == ans);
  }
}