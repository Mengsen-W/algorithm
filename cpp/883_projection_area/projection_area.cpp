/*
 * @Date: 2022-04-26 09:35:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-26 09:42:56
 * @FilePath: /algorithm/883_projection_area/projection_area.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int projectionArea(vector<vector<int>> grid) {
    int n = grid.size();
    int xyArea = 0, yzArea = 0, zxArea = 0;
    for (int i = 0; i < n; i++) {
      int yzHeight = 0, zxHeight = 0;
      for (int j = 0; j < n; j++) {
        xyArea += grid[i][j] > 0 ? 1 : 0;
        yzHeight = max(yzHeight, grid[i][j]);
        zxHeight = max(zxHeight, grid[j][i]);
      }
      yzArea += yzHeight;
      zxArea += zxHeight;
    }
    return xyArea + yzArea + zxArea;
  }
};

int main() {
  assert(Solution().projectionArea(vector<vector<int>>{{1, 2}, {3, 4}}) == 17);
  assert(Solution().projectionArea(vector<vector<int>>{{2}}) == 5);
  assert(Solution().projectionArea(vector<vector<int>>{{1, 0}, {0, 2}}) == 8);

  return 0;
}