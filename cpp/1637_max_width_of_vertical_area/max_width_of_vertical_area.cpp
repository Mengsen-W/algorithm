/*
 * @Date: 2023-03-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-30
 * @FilePath: /algorithm/cpp/1637_max_width_of_vertical_area/max_width_of_vertical_area.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxWidthOfVerticalArea(vector<vector<int>>& points) {
    sort(points.begin(), points.end());
    int mx = 0;
    for (int i = 1; i < points.size(); i++) {
      mx = max(points[i][0] - points[i - 1][0], mx);
    }
    return mx;
  }
};

int main() {
  {
    vector<vector<int>> points{{8, 7}, {9, 9}, {7, 4}, {9, 7}};
    int ans = 1;
    assert(Solution().maxWidthOfVerticalArea(points) == ans);
  }

  {
    vector<vector<int>> points{{3, 1}, {9, 0}, {1, 0}, {1, 4}, {5, 3}, {8, 8}};
    int ans = 3;
    assert(Solution().maxWidthOfVerticalArea(points) == ans);
  }
}