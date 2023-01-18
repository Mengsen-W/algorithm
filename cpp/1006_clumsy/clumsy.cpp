/*
 * @Date: 2021-04-04 18:55:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 18:58:17
 */

#include <cassert>
int clumsy(int N) {
  if (N == 1) {
    return 1;
  } else if (N == 2) {
    return 2;
  } else if (N == 3) {
    return 6;
  } else if (N == 4) {
    return 7;
  }

  if (N % 4 == 0) {
    return N + 1;
  } else if (N % 4 <= 2) {
    return N + 2;
  } else {
    return N - 1;
  }
}

int main() {
  assert(clumsy(4) == 7);
  assert(clumsy(10) == 12);
}