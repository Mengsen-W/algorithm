/*
 * @Date: 2021-10-18 08:54:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-18 09:02:54
 */

#include <cassert>

class Solution {
 public:
  int findComplement(int num) {
    int highbit = 0;
    for (int i = 1; i <= 30; ++i) {
      if (num >= (1 << i))
        highbit = i;
      else
        break;
    }
    int mask = (highbit == 30 ? 0x7fffffff : (1 << (highbit + 1)) - 1);
    return num ^ mask;
  }
};

int main() {
  assert(Solution().findComplement(5) == 2);
  assert(Solution().findComplement(1) == 0);
}