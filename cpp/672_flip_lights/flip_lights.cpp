/*
 * @Date: 2022-09-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-15
 * @FilePath: /algorithm/672_flip_lights/flip_lights.cpp
 */

#include <assert.h>

class Solution {
 public:
  int flipLights(int n, int presses) {
    if (presses > 2 && n > 2) return 8;
    if (n < 3)
      return 1 + (presses > 0) * n + (presses > 1 && n > 1);
    else
      return 1 + 3 * presses;
  }
};

int main() {
  assert(Solution().flipLights(1, 1) == 2);
  assert(Solution().flipLights(2, 1) == 3);
  assert(Solution().flipLights(3, 1) == 4);
}