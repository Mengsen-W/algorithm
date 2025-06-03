#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  long long distributeCandies(int n, int limit) {
    auto cal = [](int x) -> long long {
      if (x < 0) {
        return 0;
      }
      return (long)x * (x - 1) / 2;
    };
    return cal(n + 2) - 3 * cal(n - limit + 1) + 3 * cal(n - (limit + 1) * 2 + 2) - cal(n - 3 * (limit + 1) + 2);
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {5, 2, 3},
      {3, 3, 10},
  };

  for (auto &[n, limit, ans] : tests) {
    assert(Solution().distributeCandies(n, limit) == ans);
  }
}