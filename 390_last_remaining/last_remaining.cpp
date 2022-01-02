/*
 * @Date: 2022-01-02 01:34:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-02 01:35:33
 */

#include <cassert>

class Solution {
 public:
  int lastRemaining(int n) {
    int a1 = 1, an = n;
    int k = 0, cnt = n, step = 1;
    while (cnt > 1) {
      if (k % 2 == 0) {  // 正向
        a1 = a1 + step;
        an = (cnt % 2 == 0) ? an : an - step;
      } else {  // 反向
        a1 = (cnt % 2 == 0) ? a1 : a1 + step;
        an = an - step;
      }
      k++;
      cnt = cnt >> 1;
      step = step << 1;
    }
    return a1;
  }
};

int main() {
  assert(Solution().lastRemaining(9) == 6);
  assert(Solution().lastRemaining(1) == 1);
}
