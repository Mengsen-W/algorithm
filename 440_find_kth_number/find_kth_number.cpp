/*
 * @Date: 2022-03-23 00:37:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-23 00:42:30
 * @FilePath: /algorithm/440_find_kth_number/find_kth_number.cpp
 */

#include <cassert>
#include <cmath>

using namespace std;

class Solution {
 public:
  int getSteps(int curr, long n) {
    int steps = 0;
    long first = curr;
    long last = curr;
    while (first <= n) {
      steps += min(last, n) - first + 1;
      first = first * 10;
      last = last * 10 + 9;
    }
    return steps;
  }

  int findKthNumber(int n, int k) {
    int curr = 1;
    k--;
    while (k > 0) {
      int steps = getSteps(curr, n);
      if (steps <= k) {
        k -= steps;
        curr++;
      } else {
        curr = curr * 10;
        k--;
      }
    }
    return curr;
  }
};

int main() {
  assert(Solution().findKthNumber(13, 2) == 10);
  assert(Solution().findKthNumber(1, 1) == 1);
}