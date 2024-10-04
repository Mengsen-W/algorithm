#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  double nthPersonGetsNthSeat(int n) { return n == 1 ? 1.0 : 0.5; }
};

int main() {
  std::vector<std::tuple<int, double>> tests{
      {1, 1.000},
      {2, 0.5},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().nthPersonGetsNthSeat(n) == ans);
  }
}