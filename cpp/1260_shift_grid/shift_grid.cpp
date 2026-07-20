#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> shiftGrid(vector<vector<int>>& grid, int k) {
    int m = grid.size(), n = grid[0].size();
    vector<vector<int>> ret(m, vector<int>(n));
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        int index1 = (i * n + j + k) % (m * n);
        ret[index1 / n][index1 % n] = grid[i][j];
      }
    }
    return ret;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, vector<vector<int>>>> tests{
      {
          {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
          1,
          {{9, 1, 2}, {3, 4, 5}, {6, 7, 8}},
      },
      {
          {{3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}, {12, 0, 21, 13}},
          4,
          {{12, 0, 21, 13}, {3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}},
      },
      {
          {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
          9,
          {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
      },
  };

  for (auto [grid, k, expected] : tests) {
    assert(Solution().shiftGrid(grid, k) == expected);
  }
}