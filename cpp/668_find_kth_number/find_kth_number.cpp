/*
 * @Date: 2022-05-18 22:07:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-18 22:13:53
 * @FilePath: /algorithm/668_find_kth_number/find_kth_number.cpp
 */

#include <cassert>

class Solution {
 public:
  int findKthNumber(int m, int n, int k) {
    int left = 1, right = m * n;
    while (left < right) {
      int x = left + (right - left) / 2;
      int count = x / n * n;
      for (int i = x / n + 1; i <= m; ++i) {
        count += x / i;
      }
      if (count >= k) {
        right = x;
      } else {
        left = x + 1;
      }
    }
    return left;
  }
};

int main() {
  assert(Solution().findKthNumber(3,3,5) == 3);
  assert(Solution().findKthNumber(2,3,6) == 6);
}