#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long numberOfRightTriangles(vector<vector<int>>& grid) {
    int n = grid.size(), m = grid[0].size();
    vector<int> col(m);
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < m; j++) {
        col[j] += grid[i][j];
      }
    }
    long long res = 0;
    for (int i = 0; i < n; i++) {
      int row = accumulate(grid[i].begin(), grid[i].end(), 0);
      for (int j = 0; j < m; j++) {
        if (grid[i][j] == 1) {
          res += (row - 1) * (col[j] - 1);
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, long long>> tests{
      {{{0, 1, 0}, {0, 1, 1}, {0, 1, 0}}, 2},
      {{{1, 0, 0, 0}, {0, 1, 0, 1}, {1, 0, 0, 0}}, 0},
      {{{1, 0, 1}, {1, 0, 0}, {1, 0, 0}}, 2},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().numberOfRightTriangles(grid) == ans);
  }
}