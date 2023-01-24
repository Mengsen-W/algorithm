/*
 * @Date: 2023-01-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-24
 * @FilePath: /algorithm/cpp/1828_count_points/count_points.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> countPoints(vector<vector<int>>& points, vector<vector<int>>& queries) {
    int m = points.size(), n = queries.size();
    vector<int> ans(n);
    for (int i = 0; i < n; ++i) {
      int cx = queries[i][0], cy = queries[i][1], cr = queries[i][2];
      for (int j = 0; j < m; ++j) {
        int px = points[j][0], py = points[j][1];
        if ((cx - px) * (cx - px) + (cy - py) * (cy - py) <= cr * cr) {
          ++ans[i];
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> points{{1, 3}, {3, 3}, {5, 3}, {2, 2}};
    vector<vector<int>> queries{{2, 3, 1}, {4, 3, 1}, {1, 1, 2}};
    vector<int> ans{3, 2, 2};
    assert(Solution().countPoints(points, queries) == ans);
  }

  {
    vector<vector<int>> points{{1, 1}, {2, 2}, {3, 3}, {4, 4}, {5, 5}};
    vector<vector<int>> queries{{1, 2, 2}, {2, 2, 2}, {4, 3, 2}, {4, 3, 3}};
    vector<int> ans{2, 3, 2, 4};
    assert(Solution().countPoints(points, queries) == ans);
  }
}