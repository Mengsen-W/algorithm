/*
 * @Date: 2022-12-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-01
 * @FilePath: /algorithm/1779_nearest_valid_point/nearest_valid_point.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int nearestValidPoint(int x, int y, vector<vector<int>>& points) {
    int n = points.size();
    int best = numeric_limits<int>::max(), bestid = -1;
    for (int i = 0; i < n; ++i) {
      int px = points[i][0], py = points[i][1];
      if (x == px) {
        if (int dist = abs(y - py); dist < best) {
          best = dist;
          bestid = i;
        }
      } else if (y == py) {
        if (int dist = abs(x - px); dist < best) {
          best = dist;
          bestid = i;
        }
      }
    }
    return bestid;
  }
};

int main() {
  {
    int x = 3;
    int y = 4;
    vector<vector<int>> points{{1, 2}, {3, 1}, {2, 4}, {2, 3}, {4, 4}};
    int ans = 2;
    assert(Solution().nearestValidPoint(x, y, points) == ans);
  }

  {
    int x = 3;
    int y = 4;
    vector<vector<int>> points{{3, 4}};
    int ans = 0;
    assert(Solution().nearestValidPoint(x, y, points) == ans);
  }
}