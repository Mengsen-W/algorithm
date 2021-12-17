/*
 * @Date: 2021-12-17 07:51:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-17 07:59:49
 */

#include <algorithm>
#include <cassert>
#include <cmath>
#include <vector>

using namespace std;

class Solution {
 public:
  int visiblePoints(vector<vector<int>>& points, int angle,
                    vector<int>& location) {
    int sameCnt = 0;
    vector<double> polarDegrees;
    for (auto& point : points) {
      if (point[0] == location[0] && point[1] == location[1]) {
        sameCnt++;
        continue;
      }
      double degree = atan2(point[1] - location[1], point[0] - location[0]);
      polarDegrees.emplace_back(degree);
    }
    sort(polarDegrees.begin(), polarDegrees.end());

    int m = polarDegrees.size();
    for (int i = 0; i < m; ++i) {
      polarDegrees.emplace_back(polarDegrees[i] + 2 * M_PI);
    }

    int maxCnt = 0;
    int right = 0;
    double degree = angle * M_PI / 180;
    for (int i = 0; i < m; ++i) {
      int polarDegrees_size = polarDegrees.size();
      while (right < polarDegrees_size &&
             polarDegrees[right] <= polarDegrees[i] + degree) {
        right++;
      }
      maxCnt = max(maxCnt, right - i);
    }
    return maxCnt + sameCnt;
  }
};

int main() {
  {
    vector<vector<int>> points{{1, 2}, {2, 2}, {3, 3}};
    int angle = 90;
    vector<int> location{1, 1};
    assert(Solution().visiblePoints(points, angle, location) == 3);
  }

  {
    vector<vector<int>> points{{2, 1}, {2, 2}, {3, 4}, {1, 1}};
    int angle = 90;
    vector<int> location{1, 1};
    assert(Solution().visiblePoints(points, angle, location) == 4);
  }

  {
    vector<vector<int>> points{{1, 0}, {2, 1}};
    int angle = 13;
    vector<int> location{1, 1};
    assert(Solution().visiblePoints(points, angle, location) == 1);
  }
  return 0;
}