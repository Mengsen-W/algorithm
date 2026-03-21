#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> reverseSubmatrix(vector<vector<int>>& grid, int x, int y, int k) {
    for (int i0 = x, i1 = x + k - 1; i0 < i1; ++i0, --i1) {
      for (int j = y; j < y + k; ++j) {
        swap(grid[i0][j], grid[i1][j]);
      }
    }
    return grid;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int, int, vector<vector<int>>>> tests{
      {
          {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}, {13, 14, 15, 16}},
          1,
          0,
          3,
          {{1, 2, 3, 4}, {13, 14, 15, 8}, {9, 10, 11, 12}, {5, 6, 7, 16}},
      },
      {
          {{3, 4, 2, 3}, {2, 3, 4, 2}},
          0,
          2,
          2,
          {{3, 4, 4, 2}, {2, 3, 2, 3}},
      },
  };

  for (auto& [grid, x, y, k, expected] : tests) {
    assert(Solution().reverseSubmatrix(grid, x, y, k) == expected);
  }
}