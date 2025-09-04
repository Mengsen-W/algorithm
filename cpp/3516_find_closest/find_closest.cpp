#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findClosest(int x, int y, int z) {
    int dxz = abs(x - z), dyz = abs(y - z);
    if (dxz < dyz) {
      return 1;
    } else if (dxz > dyz) {
      return 2;
    } else {
      return 0;
    }
  }
};

int main() {
  vector<tuple<int, int, int, int>> tests{
      {2, 7, 4, 1},
      {2, 5, 6, 2},
  };

  for (auto &[x, y, z, expected] : tests) {
    assert(Solution().findClosest(x, y, z) == expected);
  }
}