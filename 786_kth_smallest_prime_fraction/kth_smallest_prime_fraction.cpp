/*
 * @Date: 2021-11-29 03:24:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-29 03:50:19
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> kthSmallestPrimeFraction(vector<int> arr, int k) {
    int n = arr.size();
    double left = 0.0, right = 1.0;
    while (true) {
      double mid = (left + right) / 2;
      int i = -1, count = 0;
      int x = 0, y = 1;

      for (int j = 1; j < n; ++j) {
        while ((double)arr[i + 1] / arr[j] < mid) {
          ++i;
          if (arr[i] * y > arr[j] * x) {
            x = arr[i];
            y = arr[j];
          }
        }
        count += i + 1;
      }

      if (count == k) return {x, y};

      if (count < k)
        left = mid;
      else
        right = mid;
    }
  }
};

int main() {
  assert((Solution().kthSmallestPrimeFraction({1, 2, 3, 5}, 3) ==
          vector<int>{2, 5}));
  assert((Solution().kthSmallestPrimeFraction({1, 7}, 1) == vector<int>{1, 7}));
}