/*
 * @Date: 2022-11-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-04
 * @FilePath: /algorithm/754_reach_number/reach_number.cpp
 */

#include <cassert>
#include <cmath>

class Solution {
 public:
  int reachNumber(int target) {
    target = abs(target);
    int k = 0;
    while (target > 0) {
      k++;
      target -= k;
    }
    return target % 2 == 0 ? k : k + 1 + k % 2;
  }
};

int main() {
  assert(Solution().reachNumber(2) == 3);
  assert(Solution().reachNumber(3) == 2);
}