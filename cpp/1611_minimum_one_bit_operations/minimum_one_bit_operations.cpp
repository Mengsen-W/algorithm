#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int minimumOneBitOperations(int n) {
    int ans = 0;
    while (n) {
      ans ^= n;
      n >>= 1;
    }
    return ans;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {3, 2},
      {6, 4},
  };

  for (auto& [n, ans] : tests) {
    assert(Solution().minimumOneBitOperations(n) == ans);
  }
}
