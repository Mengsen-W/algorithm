#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int sumOfTheDigitsOfHarshadNumber(int x) {
    int s = 0;
    for (int y = x; y; y /= 10) {
      s += y % 10;
    }
    return x % s ? -1 : s;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {18, 9},
      {23, -1},
  };

  for (auto &[x, ans] : tests) {
    assert(Solution().sumOfTheDigitsOfHarshadNumber(x) == ans);
  }
}