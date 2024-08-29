#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool satisfiesConditions(vector<vector<int>>& grid) {
    for (int i = 0; i < grid.size(); ++i) {
      for (int j = 0; j < grid[0].size(); ++j) {
        if (i + 1 < grid.size() && grid[i][j] != grid[i + 1][j]) {
          return false;
        }
        if (j + 1 < grid[0].size() && grid[i][j] == grid[i][j + 1]) {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, bool>> tests{
      {{{1, 0, 2}, {1, 0, 2}}, true},
      {{{1, 1, 1}, {0, 0, 0}}, false},
      {{{1}, {2}, {3}}, false},
  };

  for (auto &[grid, ans] : tests) {
    assert(Solution().satisfiesConditions(grid) == ans);
  }
}