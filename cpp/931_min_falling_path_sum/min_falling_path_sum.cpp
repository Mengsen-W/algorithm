/*
 * @Date: 2023-07-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-13
 * @FilePath: /algorithm/cpp/931_min_falling_path_sum/min_falling_path_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minFallingPathSum(vector<vector<int>>& matrix) {
    int n = matrix.size();
    vector<vector<int>> dp(n, vector<int>(n));
    copy(matrix[0].begin(), matrix[0].end(), dp[0].begin());
    for (int i = 1; i < n; i++) {
      for (int j = 0; j < n; j++) {
        int mn = dp[i - 1][j];
        if (j > 0) {
          mn = min(mn, dp[i - 1][j - 1]);
        }
        if (j < n - 1) {
          mn = min(mn, dp[i - 1][j + 1]);
        }
        dp[i][j] = mn + matrix[i][j];
      }
    }
    return *min_element(dp[n - 1].begin(), dp[n - 1].end());
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{2, 1, 3}, {6, 5, 4}, {7, 8, 9}}, 13},
      {{{-19, 57}, {-40, -5}}, -59},
  };

  for (auto& [matrix, ans] : tests) {
    assert(Solution().minFallingPathSum(matrix) == ans);
  }
}