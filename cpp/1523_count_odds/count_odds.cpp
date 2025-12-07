#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int pre(int x) { return (x + 1) >> 1; }
  int countOdds(int low, int high) { return pre(high) - pre(low - 1); }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {3, 7, 3},
      {8, 10, 1},
  };

  for (auto& [low, high, ans] : tests) {
    assert(Solution().countOdds(low, high) == ans);
  }
}
