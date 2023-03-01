/*
 * @Date: 2023-03-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-01
 * @FilePath: /algorithm/cpp/2373_largest_local/largest_local.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> largestLocal(vector<vector<int>>& grid) {
    int n = grid.size();
    vector<vector<int>> res(n - 2, vector<int>(n - 2, 0));
    for (int i = 0; i < n - 2; i++) {
      for (int j = 0; j < n - 2; j++) {
        for (int x = i; x < i + 3; x++) {
          for (int y = j; y < j + 3; y++) {
            res[i][j] = max(res[i][j], grid[x][y]);
          }
        }
      }
    }
    return res;
  }
};

int main() {
  {
    vector<vector<int>> grid{{9, 9, 8, 1}, {5, 6, 2, 6}, {8, 2, 6, 4}, {6, 2, 2, 2}};
    vector<vector<int>> ans{{9, 9}, {8, 6}};
    assert(Solution().largestLocal(grid) == ans);
  }

  {
    vector<vector<int>> grid{{1, 1, 1, 1, 1}, {1, 1, 1, 1, 1}, {1, 1, 2, 1, 1}, {1, 1, 1, 1, 1}, {1, 1, 1, 1, 1}};
    vector<vector<int>> ans{{2, 2, 2}, {2, 2, 2}, {2, 2, 2}};
    assert(Solution().largestLocal(grid) == ans);
  }
}
