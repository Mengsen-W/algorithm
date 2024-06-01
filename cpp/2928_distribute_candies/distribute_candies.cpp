#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int cal(int x) {
    if (x < 0) {
      return 0;
    }
    return x * (x - 1) / 2;
  }

  int distributeCandies(int n, int limit) {
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