#include <algorithm>
#include <cassert>
#include <climits>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> minAbsDiff(vector<vector<int>>& grid, int k) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> res(m - k + 1, vector<int>(n - k + 1));
    for (int i = 0; i + k <= m; i++) {
      for (int j = 0; j + k <= n; j++) {
        vector<int> kgrid;
        for (int x = i; x < i + k; x++) {
          for (int y = j; y < j + k; y++) {
            kgrid.push_back(grid[x][y]);
          }
        }
        int kmin = INT_MAX;
        sort(kgrid.begin(), kgrid.end());
        for (int t = 1; t < kgrid.size(); t++) {
          if (kgrid[t] == kgrid[t - 1]) {
            continue;
          }
          kmin = min(kmin, kgrid[t] - kgrid[t - 1]);
        }
        if (kmin != INT_MAX) {
          res[i][j] = kmin;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, vector<vector<int>>>> tests{
      {{{1, 8}, {3, -2}}, 2, {{2}}},
      {{{3, -1}}, 1, {{0, 0}}},
      {{{1, -2, 3}, {2, 3, 5}}, 2, {{1, 2}}},
  };

  for (auto [grid, k, expected] : tests) {
    assert(Solution().minAbsDiff(grid, k) == expected);
  }

  return 0;
}
