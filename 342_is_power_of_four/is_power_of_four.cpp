/*
 * @Date: 2021-05-31 09:05:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-31 09:08:39
 */

#include <cassert>

bool isPowerOfFour(int n) { return n > 0 && (n & (n - 1)) == 0 && n % 3 == 1; }

int main() {
  assert(isPowerOfFour(16));
  assert(!isPowerOfFour(5));
  assert(isPowerOfFour(1));
}
