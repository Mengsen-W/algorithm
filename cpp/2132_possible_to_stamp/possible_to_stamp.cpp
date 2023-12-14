/*
 * @Date: 2023-12-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-14
 * @FilePath: /algorithm/cpp/2132_possible_to_stamp/possible_to_stamp.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool possibleToStamp(vector<vector<int>>& grid, int stampHeight, int stampWidth) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> sum(m + 2, vector<int>(n + 2, 0));
    vector<vector<int>> diff(m + 2, vector<int>(n + 2, 0));
    for (int i = 1; i <= m; i++) {
      for (int j = 1; j <= n; j++) {
        sum[i][j] = sum[i - 1][j] + sum[i][j - 1] - sum[i - 1][j - 1] + grid[i - 1][j - 1];
      }
    }

    for (int i = 1; i + stampHeight - 1 <= m; i++) {
      for (int j = 1; j + stampWidth - 1 <= n; j++) {
        int x = i + stampHeight - 1;
        int y = j + stampWidth - 1;
        if (sum[x][y] - sum[x][j - 1] - sum[i - 1][y] + sum[i - 1][j - 1] == 0) {
          diff[i][j]++;
          diff[i][y + 1]--;
          diff[x + 1][j]--;
          diff[x + 1][y + 1]++;
        }
      }
    }

    for (int i = 1; i <= m; i++) {
      for (int j = 1; j <= n; j++) {
        diff[i][j] += diff[i - 1][j] + diff[i][j - 1] - diff[i - 1][j - 1];
        if (diff[i][j] == 0 && grid[i - 1][j - 1] == 0) {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int, bool>> tests{
      {{{1, 0, 0, 0}, {1, 0, 0, 0}, {1, 0, 0, 0}, {1, 0, 0, 0}, {1, 0, 0, 0}}, 4, 3, true},
      {{{1, 0, 0, 0}, {0, 1, 0, 0}, {0, 0, 1, 0}, {0, 0, 0, 1}}, 2,2, false},
  };

  for (auto& [grid, stampHeight, stampWidth, ans] : tests) {
    assert(Solution().possibleToStamp(grid, stampHeight, stampWidth) == ans);
  }
}