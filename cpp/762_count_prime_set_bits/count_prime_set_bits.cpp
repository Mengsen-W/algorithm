#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int countPrimeSetBits(int left, int right) {
    int ans = 0;
    for (int x = left; x <= right; ++x) {
      if ((1 << __builtin_popcount(x)) & 665772) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {6, 10, 4},
      {10, 15, 5},
  };

  for (auto& [left, right, ans] : tests) {
    assert(Solution().countPrimeSetBits(left, right) == ans);
  }
}
