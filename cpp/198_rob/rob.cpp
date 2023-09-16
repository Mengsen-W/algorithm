/*
 * @Date: 2023-09-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-16
 * @FilePath: /algorithm/cpp/198_rob/rob.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int rob(vector<int>& nums) {
    int prev = 0, curr = 0;
    for (int i : nums) {
      int temp = max(curr, prev + i);
      prev = curr;
      curr = temp;
    }
    return curr;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 1}, 4},
      {{2, 7, 9, 3, 1}, 12},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().rob(nums) == ans);
  }
}