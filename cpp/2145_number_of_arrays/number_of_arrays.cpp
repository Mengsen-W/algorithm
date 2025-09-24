#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfArrays(vector<int>& differences, int lower, int upper) {
    int x = 0, y = 0, cur = 0;
    for (int d : differences) {
      cur += d;
      x = min(x, cur);
      y = max(y, cur);
      if (y - x > upper - lower) {
        return 0;
      }
    }
    return (upper - lower) - (y - x) + 1;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int>> tests{
      {{1, -3, 4}, 1, 6, 2},
      {{3, -4, 5, 1, -2}, -4, 5, 4},
      {{4, -7, 2}, 3, 6, 0},
  };

  for (auto &[differences, lower, upper, ans] : tests) {
    assert(Solution().numberOfArrays(differences, lower, upper) == ans);
  }
}