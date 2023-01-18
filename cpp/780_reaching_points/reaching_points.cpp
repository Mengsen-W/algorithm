/*
 * @Date: 2022-04-09 07:35:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-09 07:42:18
 * @FilePath: /algorithm/780_reaching_points/reaching_points.cpp
 */

class Solution {
 public:
  bool reachingPoints(int sx, int sy, int tx, int ty) {
    while (tx > sx && ty > sy && tx != ty) {
      if (tx > ty) {
        tx %= ty;
      } else {
        ty %= tx;
      }
    }
    if (tx == sx && ty == sy) {
      return true;
    } else if (tx == sx) {
      return ty > sy && (ty - sy) % tx == 0;
    } else if (ty == sy) {
      return tx > sx && (tx - sx) % ty == 0;
    } else {
      return false;
    }
  }
};

#include <cassert>
int main() {
  assert(Solution().reachingPoints(1, 1, 3, 5) == true);
  assert(Solution().reachingPoints(1, 1, 2, 2) == false);
  assert(Solution().reachingPoints(1, 1, 1, 1) == true);
}
