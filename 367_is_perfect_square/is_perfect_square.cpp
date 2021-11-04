/*
 * @Date: 2021-11-04 00:52:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-04 00:55:48
 * @FilePath: /algorithm/367_is_perfect_square/is_perfect_square.cpp
 * @Description: file content
 */

#include <cassert>

class Solution {
 public:
  bool isPerfectSquare(int num) {
    double x0 = num;
    while (true) {
      double x1 = (x0 + num / x0) / 2;
      if (x0 - x1 < 1e-6) break;
      x0 = x1;
    }
    int x = (int)x0;
    return x * x == num;
  }
};

int main() {
  assert(Solution().isPerfectSquare(16));
  assert(!Solution().isPerfectSquare(14));
}