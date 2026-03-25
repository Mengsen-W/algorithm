#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<vector<int>> constructProductMatrix(vector<vector<int>> &grid) {
    const int MOD = 12345;
    int n = grid.size(), m = grid[0].size();
    vector<vector<int>> p(n, vector<int>(m));

    long long suffix = 1;
    for (int i = n - 1; i >= 0; i--) {
      for (int j = m - 1; j >= 0; j--) {
        p[i][j] = suffix;
        suffix = suffix * grid[i][j] % MOD;
      }
    }

    long long prefix = 1;
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < m; j++) {
        p[i][j] = p[i][j] * prefix % MOD;
        prefix = prefix * grid[i][j] % MOD;
      }
    }

    return p;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>>> tests{
      {{{1, 2}, {3, 4}}, {{24, 12}, {8, 6}}},
      {{{12345}, {2}, {1}}, {{2}, {0}, {0}}},
  };

  for (auto [grid, expected] : tests) {
    assert(Solution().constructProductMatrix(grid) == expected);
  }
}