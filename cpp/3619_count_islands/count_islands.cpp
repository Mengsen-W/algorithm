#include <cassert>
#include <functional>
#include <vector>
using namespace std;

class Solution {
public:
    int countIslands(vector<vector<int>>& grid, int k) {
        int m = grid.size(), n = grid[0].size();
        const int dirs[5] = {-1, 0, 1, 0, -1};

        function<long long(int, int)> dfs = [&](int i, int j) -> long long {
            long long sum = grid[i][j];
            grid[i][j] = 0;
            for (int d = 0; d < 4; ++d) {
                int x = i + dirs[d], y = j + dirs[d + 1];
                if (x >= 0 && x < m && y >= 0 && y < n && grid[x][y] > 0) {
                    sum += dfs(x, y);
                }
            }
            return sum;
        };

        int ans = 0;
        for (int i = 0; i < m; ++i) {
          for (int j = 0; j < n; ++j) {
            if (grid[i][j] > 0 && dfs(i, j) % k == 0) {
              ++ans;
            }
          }
        }

        return ans;
    }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {
          {
              {0, 2, 1, 0, 0},
              {0, 5, 0, 0, 5},
              {0, 0, 1, 0, 0},
              {0, 1, 4, 7, 0},
              {0, 2, 0, 0, 8},
          },
          5,
          2,
      },
      {
          {
              {3, 0, 3, 0},
              {0, 3, 0, 3},
              {3, 0, 3, 0},
          },
          3,
          6,
      },
  };

  for (auto& [grid, k, ans] : tests) {
    assert(Solution().countIslands(grid, k) == ans);
  }
}
