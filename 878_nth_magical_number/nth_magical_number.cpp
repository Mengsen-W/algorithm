/*
 * @Date: 2022-11-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-22
 * @FilePath: /algorithm/878_nth_magical_number/nth_magical_number.cpp
 */

#include <cassert>
#include <numeric>

using namespace std;

class Solution {
 public:
  const int MOD = 1e9 + 7;
  int nthMagicalNumber(int n, int a, int b) {
    int c = lcm(a, b);
    int m = c / a + c / b - 1;
    int r = n % m;
    int res = (long long)c * (n / m) % MOD;
    if (r == 0) {
      return res;
    }
    int addA = a, addB = b;
    for (int i = 0; i < r - 1; ++i) {
      if (addA < addB) {
        addA += a;
      } else {
        addB += b;
      }
    }
    return (res + min(addA, addB) % MOD) % MOD;
  }
};

int main() {
  assert(Solution().nthMagicalNumber(1, 2, 3) == 2);
  assert(Solution().nthMagicalNumber(4, 2, 3) == 6);
}