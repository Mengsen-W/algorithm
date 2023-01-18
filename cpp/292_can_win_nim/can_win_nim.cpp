/*
 * @Date: 2021-09-18 08:41:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-18 08:42:40
 */

#include <cassert>

class Solution {
 public:
  bool canWinNim(int n) { return n % 4 != 0; }
};

int main() {
  assert(!Solution().canWinNim(4));
  assert(Solution().canWinNim(1));
  assert(Solution().canWinNim(2));
}