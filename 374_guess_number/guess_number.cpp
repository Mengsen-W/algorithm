/*
 * @Date: 2021-06-14 09:36:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-14 09:46:31
 */

#include <cassert>

static int PICK = 0;

int guess(int n) {
  if (n > PICK)
    return -1;
  else if (n < PICK)
    return 1;
  else
    return 0;
}

int guessNumber(int n) {
  int left = 1, right = n;
  while (left < right) {  // 循环直至区间左右端点相同
    int mid = left + (right - left) / 2;  // 防止计算时溢出
    if (guess(mid) <= 0) {
      right = mid;  // 答案在区间 [left, mid] 中
    } else {
      left = mid + 1;  // 答案在区间 [mid+1, right] 中
    }
  }
  // 此时有 left == right，区间缩为一个点，即为答案
  return left;
}

int main() {
  {
    int n = 10;
    PICK = 6;
    assert(guessNumber(n) == PICK);
  }
  {
    int n = 1;
    PICK = 1;
    assert(guessNumber(n) == PICK);
  }
  {
    int n = 2;
    PICK = 1;
    assert(guessNumber(n) == PICK);
  }
  {
    int n = 2;
    PICK = 2;
    assert(guessNumber(n) == PICK);
  }
}