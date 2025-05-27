#include <cassert>
#include <tuple>
#include <vector>

class Solution {
public:
    int differenceOfSums(int n, int m) {
        int k = n / m;
        return n * (n + 1) / 2 - k * (k + 1) * m;
    }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {10, 3, 19},
      {5, 6, 15},
      {5, 1, -15},
  };

  for (auto &[n, m, ans] : tests) {
    assert(Solution().differenceOfSums(n, m) == ans);
  }
}