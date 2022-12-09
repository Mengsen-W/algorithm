/*
 * @Date: 2022-12-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-09
 * @FilePath: /algorithm/1780_check_powers_of_three/check_powers_of_three.cpp
 */

#include <cassert>

class Solution {
 public:
  bool checkPowersOfThree(int n) {
    while (n) {
      if (n % 3 == 2) {
        return false;
      }
      n /= 3;
    }
    return true;
  }
};

int main() {
  assert(Solution().checkPowersOfThree(12));
  assert(Solution().checkPowersOfThree(91));
  assert(!Solution().checkPowersOfThree(21));
}