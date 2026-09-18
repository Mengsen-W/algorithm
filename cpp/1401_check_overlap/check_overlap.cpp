#include <algorithm>
#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkOverlap(int radius, int xCenter, int yCenter, int x1, int y1, int x2, int y2) {
    long long dist = 0;
    if (xCenter < x1 || xCenter > x2) {
      dist += min(pow(x1 - xCenter, 2), pow(x2 - xCenter, 2));
    }
    if (yCenter < y1 || yCenter > y2) {
      dist += min(pow(y1 - yCenter, 2), pow(y2 - yCenter, 2));
    }
    return dist <= radius * radius;
  }
};

int main() {
  vector<tuple<int, int, int, int, int, int, int, bool>> tests{
      {1, 0, 0, 1, -1, 3, 1, true},
      {1, 1, 1, 1, -3, 2, -1, false},
      {1, 0, 0, -1, 0, 0, 1, true},
  };

  for (auto &[radius, xCenter, yCenter, x1, y1, x2, y2, ans] : tests) {
    assert(Solution().checkOverlap(radius, xCenter, yCenter, x1, y1, x2, y2) == ans);
  }
}