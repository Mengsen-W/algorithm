#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  bool isPowerOfThree(int n) {
    while (n && n % 3 == 0) n /= 3;
    return n == 1;
  }
};

int main() {
  std::vector<std::tuple<int, bool>> tests{
      {27, true},
      {0, false},
      {9, true},
      {45, false},
  };

  for (auto& [n, expected] : tests) {
    assert(Solution().isPowerOfThree(n) == expected);
  }
}