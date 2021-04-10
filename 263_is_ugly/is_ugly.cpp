/*
 * @Date: 2021-04-10 08:55:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-10 09:08:40
 */

#include <cassert>

bool is_ugly(int n) {
  if (n < 1) return false;
  while (n % 5 == 0) {
    n /= 5;
  }
  while (n % 3 == 0) {
    n /= 3;
  }
  while (n % 2 == 0) {
    n >>= 1;
  }
  return n == 1;
}

int main() {
  assert(is_ugly(6));
  assert(is_ugly(8));
  assert(!is_ugly(14));
  assert(is_ugly(1));
  return 0;
}