/*
 * @Date: 2021-06-11 08:30:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-11 08:39:59
 */

#include <cassert>
#include <cmath>
#include <functional>

using namespace std;

int numSquares(int n) {
  // 判断是否为完全平方数
  function<bool(int)> isPerfectSquare = [](int x) -> bool {
    int y = sqrt(x);
    return y * y == x;
  };

  // 判断是否能表示为 4^k*(8m+7)
  function<bool(int)> checkAnswer4 = [](int x) -> bool {
    while (x % 4 == 0) {
      x /= 4;
    }
    return x % 8 == 7;
  };
  if (isPerfectSquare(n)) {
    return 1;
  }
  if (checkAnswer4(n)) {
    return 4;
  }
  for (int i = 1; i * i <= n; i++) {
    int j = n - i * i;
    if (isPerfectSquare(j)) {
      return 2;
    }
  }
  return 3;
}

int main() {
  assert(numSquares(12) == 3);
  assert(numSquares(13) == 2);
  return 0;
}