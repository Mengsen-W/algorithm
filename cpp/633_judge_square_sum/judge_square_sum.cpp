/*
 * @Date: 2021-04-28 09:33:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-28 09:56:16
 */

#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

class Solution {
 public:
  bool judgeSquareSum(int c) {
    long left = 0;
    long right = (int)sqrt(c);
    while (left <= right) {
      long sum = left * left + right * right;
      if (sum > c)
        --right;
      else if (sum < c)
        ++left;
      else
        return true;
    }
    return false;
  }
};

int main() {
  std::vector<std::tuple<int, bool>> tests{
      {5, true},
      {3, false},
  };

  for (auto &[c, ans] : tests) {
    assert(Solution().judgeSquareSum(c) == ans);
  }

  return 0;
}