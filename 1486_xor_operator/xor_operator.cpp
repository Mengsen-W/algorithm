/*
 * @Date: 2021-05-07 09:54:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-07 10:13:21
 */

#include <cassert>

int xor_operation(int n, int start) {
  int ans = 0;

  for (int i = 0; i < n; ++i) ans ^= (start + i * 2);

  return ans;
}

int main() {
  assert(xor_operation(5, 0) == 8);
  assert(xor_operation(4, 3) == 8);
  assert(xor_operation(1, 7) == 7);
  assert(xor_operation(10, 5) == 2);
  return 0;
}