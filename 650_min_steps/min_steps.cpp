/*
 * @Date: 2021-09-19 09:01:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-19 09:04:45
 */

#include <cassert>
#include <climits>
#include <vector>

using namespace std;

class Solution {
 public:
  int minSteps(int n) {
    vector<int> f(n + 1);
    for (int i = 2; i <= n; ++i) {
      f[i] = INT_MAX;
      for (int j = 1; j * j <= i; ++j) {
        if (i % j == 0) {
          f[i] = min(f[i], f[j] + i / j);
          f[i] = min(f[i], f[i / j] + j);
        }
      }
    }
    return f[n];
  }
};

int main() {
  assert(Solution().minSteps(3) == 3);
  assert(Solution().minSteps(1) == 0);
}