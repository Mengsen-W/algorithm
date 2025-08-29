#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  long long flowerGame(int n, int m) { return (long long)m * n / 2; }
};

int main() {
  std::vector<std::tuple<int, int, long long>> tests{
      {3, 2, 3},
      {1, 1, 0},
  };

  for (auto &[n, m, expected] : tests) {
    assert(Solution().flowerGame(n, m) == expected);
  }
}