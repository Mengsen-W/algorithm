#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 private:
  static constexpr int mod = 1000000007;

 public:
  int countGoodNumbers(long long n) {
    // 快速幂求出 x^y % mod
    auto quickmul = [](int x, long long y) -> int {
      int ret = 1, mul = x;
      while (y > 0) {
        if (y % 2 == 1) {
          ret = (long long)ret * mul % mod;
        }
        mul = (long long)mul * mul % mod;
        y /= 2;
      }
      return ret;
    };

    return (long long)quickmul(5, (n + 1) / 2) * quickmul(4, n / 2) % mod;
  }
};

int main() {
  std::vector<std::tuple<long long, int>> tests{
      {1, 5},
      {4, 400},
      {50, 564908303},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().countGoodNumbers(n) == ans);
  }
}