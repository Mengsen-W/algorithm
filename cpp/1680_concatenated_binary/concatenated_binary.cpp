#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int concatenatedBinary(int n) {
    constexpr long long MOD = 1e9 + 7;
    int suf_len = 0;  // 后面的数积累的二进制总长
    long long ans = 0;
    while (n > 0) {
      int l = 1 << __lg(n);  // l是个2^i形式的数，是此轮的组的下界。此轮的组为 [l, n]

      int k = __lg(l) + 1;  // 此组的二进制长度

      // 计算 [l, n] 的 S，[n - (n - l), n]
      long long a = n, t = n - l;
      // 一部分一部分地算，以下是计算S的过程
      long long p1 = (a - t) * fast_pow(2, (t + 1) * k) % MOD;
      long long p2 = fast_pow(2, k) * (fast_pow(2, t * k) - 1) % MOD;
      long long p3 = fast_pow(2, k) - 1;
      long long p3_inv = fast_pow(p3, MOD - 2);  // 逆元
      long long S = (p1 + p2 * p3_inv % MOD - a + MOD) * p3_inv % MOD;

      S = S * fast_pow(2, suf_len) % MOD;

      ans = (ans + S) % MOD;
      suf_len += (t + 1) * k;  // 后缀多了 t + 1 个二进制长为 k 的数，加到 suf_len 里
      n = l - 1;
    }
    return ans;
  }

  long long fast_pow(long long a, long long b) {
    constexpr long long MOD = 1e9 + 7;
    long long res = 1;
    while (b) {
      if (b & 1) res = res * a % MOD;
      a = a * a % MOD;
      b >>= 1;
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {1, 1},
      {3, 27},
      {12, 505379714},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().concatenatedBinary(n) == expected);
  }
}