/*
 * @Date: 2023-07-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-12
 * @FilePath: /algorithm/cpp/2544_alternate_digit_sum/alternate_digit_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int alternateDigitSum(int n) {
    int res = 0, sign = 1;
    while (n > 0) {
      res += n % 10 * sign;
      sign = -sign;
      n /= 10;
    }
    return -sign * res;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{{521, 4}, {111, 1}, {886996, 0}};
  for (auto &[n, ans] : tests) {
    assert(Solution().alternateDigitSum(n) == ans);
  }
}