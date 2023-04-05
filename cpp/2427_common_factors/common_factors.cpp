/*
 * @Date: 2023-04-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-05
 * @FilePath: /algorithm/cpp/2427_common_factors/common_factors.cpp
 */

#include <cassert>
#include <numeric>

using namespace std;

class Solution {
 public:
  int commonFactors(int a, int b) {
    int c = gcd(a, b), ans = 0;
    for (int x = 1; x * x <= c; ++x) {
      if (c % x == 0) {
        ++ans;
        if (x * x != c) {
          ++ans;
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    int a = 12, b = 6, ans = 4;
    assert(Solution().commonFactors(a, b) == ans);
  }

  {
    int a = 25, b = 30, ans = 2;
    assert(Solution().commonFactors(a, b) == ans);
  }
}