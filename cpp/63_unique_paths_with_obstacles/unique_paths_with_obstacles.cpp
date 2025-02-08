#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int uniquePathsWithObstacles(vector<vector<int>>& obstacleGrid) {
    int n = obstacleGrid.size(), m = obstacleGrid.at(0).size();
    vector<int> f(m);

    f[0] = (obstacleGrid[0][0] == 0);
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < m; ++j) {
        if (obstacleGrid[i][j] == 1) {
          f[j] = 0;
          continue;
        }
        if (j - 1 >= 0 && obstacleGrid[i][j - 1] == 0) {
          f[j] += f[j - 1];
        }
      }
    }

    return f.back();
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}}, 2},
      {{{0, 1}, {0, 0}}, 1},
  };
  
  for (auto &[obstacleGrid, ans] : tests) {
    assert(Solution().uniquePathsWithObstacles(obstacleGrid) == ans);
  }
}