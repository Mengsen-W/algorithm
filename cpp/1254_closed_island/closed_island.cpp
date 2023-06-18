/*
 * @Date: 2023-06-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-18
 * @FilePath: /algorithm/cpp/1254_closed_island/closed_island.cpp
 */

#include <cassert>
#include <vector>
#include <functional>

using namespace std;

class Solution {
 public:
  int closedIsland(vector<vector<int>>& grid) {
    int ans = 0;
    int m = grid.size();
    int n = grid[0].size();

    function<bool(int, int)> dfs = [&](int x, int y) -> bool {
      if (x < 0 || y < 0 || x >= m || y >= n) {
        return false;
      }
      if (grid[x][y] != 0) {
        return true;
      }
      grid[x][y] = -1;
      bool ret1 = dfs(x - 1, y);
      bool ret2 = dfs(x + 1, y);
      bool ret3 = dfs(x, y - 1);
      bool ret4 = dfs(x, y + 1);
      return ret1 && ret2 && ret3 && ret4;
    };

    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (grid[i][j] == 0 && dfs(i, j)) {
          ans++;
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> grid{{1, 1, 1, 1, 1, 1, 1, 0},
                             {1, 0, 0, 0, 0, 1, 1, 0},
                             {1, 0, 1, 0, 1, 1, 1, 0},
                             {1, 0, 0, 0, 0, 1, 0, 1},
                             {1, 1, 1, 1, 1, 1, 1, 0}};
    int ans = 2;
    assert(Solution().closedIsland(grid) == ans);
  }

  {
    vector<vector<int>> grid{{0, 0, 1, 0, 0}, {0, 1, 0, 1, 0}, {0, 1, 1, 1, 0}};
    int ans = 1;
    assert(Solution().closedIsland(grid) == ans);
  }

  {
    vector<vector<int>> grid{{1, 1, 1, 1, 1, 1, 1}, {1, 0, 0, 0, 0, 0, 1}, {1, 0, 1, 1, 1, 0, 1}, {1, 0, 1, 0, 1, 0, 1},
                             {1, 0, 1, 1, 1, 0, 1}, {1, 0, 0, 0, 0, 0, 1}, {1, 1, 1, 1, 1, 1, 1}};
    int ans = 2;
    assert(Solution().closedIsland(grid) == ans);
  }
}