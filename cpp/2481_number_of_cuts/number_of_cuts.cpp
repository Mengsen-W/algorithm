/*
 * @Date: 2023-06-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-17
 * @FilePath: /algorithm/cpp/2481_number_of_cuts/number_of_cuts.cpp
 */

#include <cassert>

class Solution {
 public:
  int numberOfCuts(int n) {
    if (n == 1) {
      return 0;
    }
    if (n % 2 == 0) {
      return n / 2;
    }
    return n;
  }
};

int main() {
  assert(Solution().numberOfCuts(4) == 2);
  assert(Solution().numberOfCuts(3) == 3);
}