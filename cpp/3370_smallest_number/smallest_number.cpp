#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int smallestNumber(int n) {
    int x = 1;
    while (x < n) {
      x = x * 2 + 1;
    }
    return x;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {5, 7},
      {10, 15},
      {3, 3},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().smallestNumber(n) == ans);
  }
}