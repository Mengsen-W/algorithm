#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxScore(vector<vector<int>>& grid) {
    int m = grid.size(), n = grid[0].size();
    vector<int> precol(n, INT_MIN);
    int ans = INT_MIN;
    for (int i = 0; i < m; ++i) {
      int prerow = INT_MIN;
      for (int j = 0; j < n; ++j) {
        int f = INT_MIN;
        if (i > 0) {
          f = max(f, grid[i][j] + precol[j]);
        }
        if (j > 0) {
          f = max(f, grid[i][j] + prerow);
        }
        ans = max(ans, f);
        precol[j] = max(precol[j], max(f, 0) - grid[i][j]);
        prerow = max(prerow, max(f, 0) - grid[i][j]);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{9, 5, 7, 3}, {8, 9, 6, 1}, {6, 7, 14, 3}, {2, 5, 3, 1}}, 9},
      {{{4, 3, 2}, {3, 2, 1}}, -1},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().maxScore(grid) == ans);
  }
}