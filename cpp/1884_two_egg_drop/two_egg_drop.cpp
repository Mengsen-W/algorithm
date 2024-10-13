#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int twoEggDrop(int n) { return ceil((-1 + sqrt(1 + 8 * n)) / 2); }
};

int main() {
  vector<tuple<int, int>> tests{
      {2, 2},
      {100, 14},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().twoEggDrop(n) == ans);
  }
}