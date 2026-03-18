#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countSubmatrices(vector<vector<int>>& grid, int k) {
    int n = grid.size(), m = grid[0].size();
    vector<int> cols(m);
    int res = 0;
    for (int i = 0; i < n; i++) {
      int rows = 0;
      for (int j = 0; j < m; j++) {
        cols[j] += grid[i][j];
        rows += cols[j];
        if (rows <= k) {
          res++;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{7, 6, 3}, {6, 6, 1}}, 18, 4},
      {{{7, 2, 9}, {1, 5, 0}, {2, 6, 6}}, 20, 6},
  };

  for (auto [grid, k, expected] : tests) {
    assert(Solution().countSubmatrices(grid, k) == expected);
  }
}