/*
 * @Date: 2022-03-03 00:27:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-03 00:36:16
 * @FilePath: /algorithm/258_add_digits/add_digits.cpp
 */

#include <cassert>

class Solution1 {
 public:
  int addDigits(int num) {
    while (num >= 10) {
      int sum = 0;
      while (num > 0) {
        sum += num % 10;
        num /= 10;
      }
      num = sum;
    }
    return num;
  }
};

class Solution2 {
 public:
  int addDigits(int num) { return (num - 1) % 9 + 1; }
};

int main() {
  assert(Solution1().addDigits(38) == 2);
  assert(Solution2().addDigits(38) == 2);

  assert(Solution1().addDigits(0) == 0);
  assert(Solution2().addDigits(0) == 0);
}