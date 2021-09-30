/*
 * @Date: 2021-09-30 08:34:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-30 09:26:58
 */

#include <algorithm>
#include <cassert>

using namespace std;

class Solution {
 public:
  int computeArea(int ax1, int ay1, int ax2, int ay2, int bx1, int by1, int bx2,
                  int by2) {
    int area1 = (ax2 - ax1) * (ay2 - ay1), area2 = (bx2 - bx1) * (by2 - by1);
    int overlapWidth = min(ax2, bx2) - max(ax1, bx1),
        overlapHeight = min(ay2, by2) - max(ay1, by1);
    int overlapArea = max(overlapWidth, 0) * max(overlapHeight, 0);
    return area1 + area2 - overlapArea;
  }
};

int main() {
  {
    int ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9,
        by2 = 2;
    int ans = 45;
    assert(Solution().computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) ==
           ans);
  }
  {
    int ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2,
        by2 = 2;
    int ans = 16;
    assert(Solution().computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) ==
           ans);
  }
}