#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countSquares(vector<vector<int>>& matrix) {
    int m = matrix.size(), n = matrix[0].size();
    vector<vector<int>> f(m, vector<int>(n, 0));
    int ans = 0;
    for (int i = 0; i < m; ++i) {
      for (int j = 0; j < n; ++j) {
        if (i == 0 || j == 0) {
          f[i][j] = matrix[i][j];
        } else if (matrix[i][j] == 0) {
          f[i][j] = 0;
        } else {
          f[i][j] = min({f[i][j - 1], f[i - 1][j], f[i - 1][j - 1]}) + 1;
        }
        ans += f[i][j];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{0, 1, 1, 1}, {1, 1, 1, 1}, {0, 1, 1, 1}}, 15},
      {{{1, 0, 1}, {1, 1, 0}, {1, 1, 0}}, 7},
  };

  for (auto& [matrix, expect] : tests) {
    assert(Solution().countSquares(matrix) == expect);
  }
}