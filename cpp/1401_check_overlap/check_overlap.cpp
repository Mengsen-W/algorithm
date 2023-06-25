/*
 * @Date: 2023-06-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-25
 * @FilePath: /algorithm/cpp/1401_check_overlap/check_overlap.cpp
 */

#include <algorithm>
#include <cassert>
#include <cmath>

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
  {
    int radius = 1, xCenter = 0, yCenter = 0, x1 = 1, y1 = -1, x2 = 3, y2 = 1;
    assert(Solution().checkOverlap(radius, xCenter, yCenter, x1, y1, x2, y2) == true);
  }

  {
    int radius = 1, xCenter = 1, yCenter = 1, x1 = 1, y1 = -3, x2 = 2, y2 = -1;
    assert(Solution().checkOverlap(radius, xCenter, yCenter, x1, y1, x2, y2) == false);
  }

  {
    int radius = 1, xCenter = 0, yCenter = 0, x1 = -1, y1 = 0, x2 = 0, y2 = 1;
    assert(Solution().checkOverlap(radius, xCenter, yCenter, x1, y1, x2, y2) == true);
  }
}