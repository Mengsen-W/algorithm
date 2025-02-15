#include <cassert>
#include <tuple>
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
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{1, 1, 1, -1, -1}, {1, 1, 1, -1, -1}, {-1, -1, -1, 1, 1}, {1, 1, 1, 1, -1}, {-1, -1, -1, -1, -1}},
       {1, -1, -1, -1, -1}},
      {{{-1}}, {-1}},
      {{{1, 1, 1, 1, 1, 1}, {-1, -1, -1, -1, -1, -1}, {1, 1, 1, 1, 1, 1}, {-1, -1, -1, -1, -1, -1}},
       {0, 1, 2, 3, 4, -1}},
  };
  
  for (auto &[grid, ans] : tests) {
    assert(Solution().findBall(grid) == ans);
  }
}