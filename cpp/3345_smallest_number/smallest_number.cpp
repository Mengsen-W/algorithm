#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int smallestNumber(int n, int t) {
    const auto getMultiply = [](int a) -> int {
      int ans = 1;
      while (a) {
        int tmp = a % 10;
        if (a == 0) {
          return 0;
        }
        ans *= tmp % 10;
        a /= 10;
      }
      return ans;
    };

    while (n > 0) {
      int tmp = getMultiply(n);
      if (tmp % t == 0) {
        return n;
      }
      n++;
    }

    return -1;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {10, 2, 10},
      {15, 3, 16},
  };

  for (auto [n, t, expected] : tests) {
    assert(Solution().smallestNumber(n, t) == expected);
  }
}