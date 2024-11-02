#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int minChanges(int n, int k) { return (n & k) == k ? __builtin_popcount(n ^ k) : -1; }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {13, 4, 2},
      {21, 21, 0},
      {14, 13, -1},
  };

  for (auto &[n,k,ans] : tests) {
    assert(Solution().minChanges(n, k) == ans);
  }
}
