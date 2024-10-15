#include <algorithm>
#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxHeightOfTriangle(int red, int blue) {
    auto maxHeight = [](int x, int y) -> int {
      int odd = 2 * int(sqrt(x)) - 1;
      int even = 2 * int((-1 + sqrt(1 + 4 * y)) / 2);
      return min(odd, even) + 1;
    };
    return max(maxHeight(red, blue), maxHeight(blue, red));
  }
};

int main() {
  vector<tuple<int, int, int>> tests{
      {2, 4, 3},
      {2, 1, 2},
      {1, 1, 1},
      {10, 1, 2},
  };

  for (auto &[red, blue, ans] : tests) {
    assert(Solution().maxHeightOfTriangle(red, blue) == ans);
  }
}