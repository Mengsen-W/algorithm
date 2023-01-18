/*
 * @Date: 2022-11-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-02
 * @FilePath: /algorithm/1620_best_coordinate/best_coordinate.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> bestCoordinate(vector<vector<int>> &towers, int radius) {
    int xMax = INT_MIN, yMax = INT_MIN;
    for (auto &&tower : towers) {
      int x = tower[0], y = tower[1];
      xMax = max(xMax, x);
      yMax = max(yMax, y);
    }
    int cx = 0, cy = 0;
    int maxQuality = 0;
    for (int x = 0; x <= xMax; x++) {
      for (int y = 0; y <= yMax; y++) {
        vector<int> coordinate = {x, y};
        int quality = 0;
        for (auto &&tower : towers) {
          int squaredDistance = getSquaredDistance(coordinate, tower);
          if (squaredDistance <= radius * radius) {
            double distance = sqrt((double)squaredDistance);
            quality += floor((double)tower[2] / (1 + distance));
          }
        }
        if (quality > maxQuality) {
          cx = x;
          cy = y;
          maxQuality = quality;
        }
      }
    }
    return {cx, cy};
  }

  int getSquaredDistance(const vector<int> &coordinate, const vector<int> &tower) {
    return (tower[0] - coordinate[0]) * (tower[0] - coordinate[0]) +
           (tower[1] - coordinate[1]) * (tower[1] - coordinate[1]);
  }
};

int main() {
  {
    vector<vector<int>> towers{{1, 2, 5}, {2, 1, 7}, {3, 1, 9}};
    int radius = 2;
    vector<int> ans{2, 1};
    assert(Solution().bestCoordinate(towers, radius) == ans);
  }

  {
    vector<vector<int>> towers{{23, 11, 21}};
    int radius = 9;
    vector<int> ans{23, 11};
    assert(Solution().bestCoordinate(towers, radius) == ans);
  }

  {
    vector<vector<int>> towers{{1, 2, 13}, {2, 1, 7}, {0, 1, 9}};
    int radius = 2;
    vector<int> ans{1, 2};
    assert(Solution().bestCoordinate(towers, radius) == ans);
  }
}