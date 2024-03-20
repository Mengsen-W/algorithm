/*
 * @Date: 2024-03-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-20
 * @FilePath: /algorithm/cpp/1969_min_non_zero_product/min_non_zero_product.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  long long fastPow(long long x, long long n, long long mod) {
    long long res = 1;
    for (; n != 0; n >>= 1) {
      if (n & 1) {
        res = res * x % mod;
      }
      x = x * x % mod;
    }
    return res;
  }

  int minNonZeroProduct(int p) {
    if (p == 1) {
      return 1;
    }
    long long mod = 1e9 + 7;
    long long x = fastPow(2, p, mod) - 1;
    long long y = (long long)1 << (p - 1);
    return fastPow(x - 1, y - 1, mod) * x % mod;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {1, 1},
      {2, 6},
      {3, 1512},
  };

  for (auto &[p, ans] : tests) {
    assert(Solution().minNonZeroProduct(p) == ans);
  }
}