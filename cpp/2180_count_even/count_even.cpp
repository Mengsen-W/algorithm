/*
 * @Date: 2023-01-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-06
 * @FilePath: /algorithm/2180_count_even/count_even.cpp
 */

#include <cassert>

using namespace std;

class Solution {
 public:
  int countEven(int num) {
    int y = num / 10, x = num % 10;
    int res = y * 5, ySum = 0;
    while (y) {
      ySum += y % 10;
      y /= 10;
    }
    if (ySum % 2 == 0) {
      res += x / 2 + 1;
    } else {
      res += (x + 1) / 2;
    }
    return res - 1;
  }
};

int main() {
  {
    int num = 4;
    int ans = 2;
    assert(Solution().countEven(num) == ans);
  }

  {
    int num = 30;
    int ans = 14;
    assert(Solution().countEven(num) == ans);
  }
}