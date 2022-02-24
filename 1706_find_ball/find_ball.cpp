/*
 * @Date: 2022-02-23 23:51:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-24 00:14:34
 * @FilePath: /algorithm/1706_find_ball/find_ball.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findBall(vector<vector<int>> &grid) {
    int n = grid[0].size();
    vector<int> ans(n, -1);
    for (int j = 0; j < n; ++j) {
      int col = j;  // 球的初始列
      for (auto &row : grid) {
        int dir = row[col];
        col += dir;                                    // 移动球
        if (col < 0 || col == n || row[col] != dir) {  // 到达侧边或 V 形
          col = -1;
          break;
        }
      }
      if (col >= 0) {  // 成功到达底部
        ans[j] = col;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> grid{{1, 1, 1, -1, -1},
                             {1, 1, 1, -1, -1},
                             {-1, -1, -1, 1, 1},
                             {1, 1, 1, 1, -1},
                             {-1, -1, -1, -1, -1}};
    vector<int> ans{1, -1, -1, -1, -1};
    assert(Solution().findBall(grid) == ans);
  }

  {
    vector<vector<int>> grid{{-1}};
    vector<int> ans{-1};
    assert(Solution().findBall(grid) == ans);
  }

  {
    vector<vector<int>> grid{{1, 1, 1, 1, 1, 1},
                             {-1, -1, -1, -1, -1, -1},
                             {1, 1, 1, 1, 1, 1},
                             {-1, -1, -1, -1, -1, -1}};
    vector<int> ans{0, 1, 2, 3, 4, -1};
    assert(Solution().findBall(grid) == ans);
  }
}