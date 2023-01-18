/*
 * @Date: 2022-02-03 14:07:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-03 14:47:08
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findMinFibonacciNumbers(int k) {
    vector<int> f;
    f.emplace_back(1);
    int a = 1, b = 1;
    while (a + b <= k) {
      int c = a + b;
      f.emplace_back(c);
      a = b;
      b = c;
    }

    int ans = 0;
    for (int i = f.size() - 1; i >= 0 && k > 0; i--) {
      int num = f[i];
      if (k >= num) {
        k -= num;
        ans++;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().findMinFibonacciNumbers(7) == 2);
  // assert(Solution().findMinFibonacciNumbers(10) == 2);
  // assert(Solution().findMinFibonacciNumbers(19) == 3);
}