/*
 * @Date: 2021-04-28 09:33:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-28 09:56:16
 */

#include <cassert>
#include <cmath>

bool judge_square_sum(int c) {
  long left = 0;
  long right = (int)sqrt(c);
  while (left <= right) {
    long sum = left * left + right * right;
    if (sum > c)
      --right;
    else if (sum < c)
      ++left;
    else
      return true;
  }
  return false;
}

int main() {
  {
    int c = 5;
    assert(judge_square_sum(c));
  }
  {
    int c = 3;
    assert(!judge_square_sum(c));
  }
  {
    int c = 4;
    assert(judge_square_sum(c));
  }
  {
    int c = 2;
    assert(judge_square_sum(c));
  }
  {
    int c = 1;
    assert(judge_square_sum(c));
  }
  return 0;
}