/*
 * @Date: 2022-05-15 14:54:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-15 15:52:05
 * @FilePath: /algorithm/812_largest_triangle_area/largest_triangle_area.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 private:
  int cross(const vector<int>& p, const vector<int>& q, const vector<int>& r) {
    return (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0]);
  }

  vector<vector<int>> getConvexHull(vector<vector<int>>& points) {
    int n = points.size();
    if (n < 4) {
      return points;
    }
    /* 按照 x 大小进行排序，如果 x 相同，则按照 y 的大小进行排序 */
    sort(points.begin(), points.end(), [](const vector<int>& a, const vector<int>& b) {
      if (a[0] == b[0]) {
        return a[1] < b[1];
      }
      return a[0] < b[0];
    });
    vector<vector<int>> hull;
    /* 求出凸包的下半部分 */
    for (int i = 0; i < n; i++) {
      while (hull.size() > 1 && cross(hull[hull.size() - 2], hull.back(), points[i]) <= 0) {
        hull.pop_back();
      }
      hull.emplace_back(points[i]);
    }
    int m = hull.size();
    /* 求出凸包的上半部分 */
    for (int i = n - 2; i >= 0; i--) {
      while (hull.size() > m && cross(hull[hull.size() - 2], hull.back(), points[i]) <= 0) {
        hull.pop_back();
      }
      hull.emplace_back(points[i]);
    }
    /* hull[0] 同时参与凸包的上半部分检测，因此需去掉重复的 hull[0] */
    hull.pop_back();
    return hull;
  }

  double triangleArea(int x1, int y1, int x2, int y2, int x3, int y3) {
    return 0.5 * abs(x1 * y2 + x2 * y3 + x3 * y1 - x1 * y3 - x2 * y1 - x3 * y2);
  }

 public:
  double largestTriangleArea(vector<vector<int>> points) {
    auto convexHull = getConvexHull(points);
    int n = convexHull.size();
    double ret = 0.0;
    for (int i = 0; i < n; i++) {
      for (int j = i + 1, k = i + 2; j + 1 < n; j++) {
        while (k + 1 < n) {
          double curArea = triangleArea(convexHull[i][0], convexHull[i][1], convexHull[j][0], convexHull[j][1],
                                        convexHull[k][0], convexHull[k][1]);
          double nextArea = triangleArea(convexHull[i][0], convexHull[i][1], convexHull[j][0], convexHull[j][1],
                                         convexHull[k + 1][0], convexHull[k + 1][1]);
          if (curArea >= nextArea) {
            break;
          }
          k++;
        }
        double area = triangleArea(convexHull[i][0], convexHull[i][1], convexHull[j][0], convexHull[j][1],
                                   convexHull[k][0], convexHull[k][1]);
        ret = max(ret, area);
      }
    }
    return ret;
  }
};

int main() { assert(Solution().largestTriangleArea({{0, 0}, {0, 1}, {1, 0}, {0, 2}, {2, 0}}) == 2); }