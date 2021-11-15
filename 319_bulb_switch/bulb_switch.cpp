/*
 * @Date: 2021-11-15 01:29:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-15 01:33:43
 */

#include <cassert>
#include <cmath>

class Solution {
 public:
  int bulbSwitch(int n) { return sqrt(n + 0.5); }
};

int main() {
  assert(Solution().bulbSwitch(0) == 0);
  assert(Solution().bulbSwitch(1) == 1);
  assert(Solution().bulbSwitch(3) == 1);
}