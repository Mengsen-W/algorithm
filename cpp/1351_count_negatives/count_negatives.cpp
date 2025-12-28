#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countNegatives(vector<vector<int>>& grid) {
    int num = 0;
    int m = (int)grid[0].size();
    int pos = (int)grid[0].size() - 1;

    for (auto& row : grid) {
      int i;
      for (i = pos; i >= 0; --i) {
        if (row[i] >= 0) {
          if (i + 1 < m) {
            pos = i + 1;
            num += m - pos;
          }
          break;
        }
      }
      if (i == -1) {
        num += m;
        pos = -1;
      }
    }

    return num;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{4, 3, 2, -1}, {3, 2, 1, -1}, {1, 1, -1, -2}, {-1, -1, -2, -3}}, 8},
      {{{3, 2}, {1, 0}}, 0},
  };

  for (auto& [grid, ans] : tests) {
    assert(Solution().countNegatives(grid) == ans);
  }
}