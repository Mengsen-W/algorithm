#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  bool isPowerOfFour(int n) { return n > 0 && (n & (n - 1)) == 0 && n % 3 == 1; }
};

int main() {
  std::vector<std::tuple<int, bool>> tests{
      {16, true},
      {5, false},
      {1, true},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution{}.isPowerOfFour(n) == ans);
  }
}
