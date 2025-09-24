#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfWays(int n, int x) {
    long long mod = 1e9 + 7;
    vector<long long> dp(n + 1);
    dp[0] = 1;
    for (int i = 1; i <= n; i++) {
      long long val = pow(i, x);
      if (val > n) {
        break;
      }
      for (int j = n; j >= val; j--) {
        dp[j] = (dp[j] + dp[j - val]) % mod;
      }
    }
    return dp[n];
  }
};

int main() {
  vector<tuple<int, int, int>> tests{
      {10, 2, 1},
      {4, 1, 2},
  };

  for (auto [n, x, expected] : tests) {
    assert(Solution().numberOfWays(n, x) == expected);
  }
}