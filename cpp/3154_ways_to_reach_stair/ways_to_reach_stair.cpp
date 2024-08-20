#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int waysToReachStair(int k) {
    auto comb = [](int n, int k) -> int {
      long long ans = 1;
      for (int i = n; i >= n - k + 1; --i) {
        ans *= i;
        ans /= n - i + 1;
      }
      return ans;
    };

    int n = 0, npow = 1, ans = 0;
    while (true) {
      if (npow - n - 1 <= k && k <= npow) {
        ans += comb(n + 1, npow - k);
      } else if (npow - n - 1 > k) {
        break;
      }
      ++n;
      npow *= 2;
    }
    return ans;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{{0, 2}, {1, 4}};
  for (auto &[k, ans] : tests) {
    assert(Solution().waysToReachStair(k) == ans);
  }
}