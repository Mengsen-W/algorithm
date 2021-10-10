/*
 * @Date: 2021-10-10 09:26:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-10 09:46:22
 */

#include <cassert>
#include <cmath>

class Solution {
 public:
  int arrangeCoins(int n) {
    return (int)((sqrt((long long)8 * n + 1) - 1) / 2);
  }
  // int arrangeCoins(int n) {
  //   int left = 1, right = n;
  //   while (left < right) {
  //     int mid = (right - left + 1) / 2 + left;
  //     if ((long long)mid * (mid + 1) <= (long long)2 * n) {
  //       left = mid;
  //     } else {
  //       right = mid - 1;
  //     }
  //   }
  //   return left;
  // }
};

int main() {
  assert(Solution().arrangeCoins(5) == 2);
  assert(Solution().arrangeCoins(8) == 3);
}