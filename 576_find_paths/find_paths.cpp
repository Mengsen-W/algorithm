/*
 * @Date: 2021-08-15 14:20:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-15 14:39:10
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  static constexpr int MOD = 1'000'000'007;

  int findPaths(int m, int n, int maxMove, int startRow, int startColumn) {
    vector<vector<int>> directions = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
    int outCounts = 0;
    vector<vector<int>> dp(m, vector<int>(n));
    dp[startRow][startColumn] = 1;
    for (int i = 0; i < maxMove; i++) {
      vector<vector<int>> dpNew(m, vector<int>(n));
      for (int j = 0; j < m; j++) {
        for (int k = 0; k < n; k++) {
          int count = dp[j][k];
          if (count > 0) {
            for (auto& direction : directions) {
              int j1 = j + direction[0], k1 = k + direction[1];
              if (j1 >= 0 && j1 < m && k1 >= 0 && k1 < n) {
                dpNew[j1][k1] = (dpNew[j1][k1] + count) % MOD;
              } else {
                outCounts = (outCounts + count) % MOD;
              }
            }
          }
        }
      }
      dp = dpNew;
    }
    return outCounts;
  }
};

int main() {
  {
    int m = 2;
    int n = 2;
    int maxMove = 2;
    int startRow = 0;
    int startColumn = 0;
    int ans = 6;
    assert(Solution{}.findPaths(m, n, maxMove, startRow, startColumn) == ans);
  }
  {
    int m = 1;
    int n = 3;
    int maxMove = 3;
    int startRow = 0;
    int startColumn = 1;
    int ans = 12;
    assert(Solution{}.findPaths(m, n, maxMove, startRow, startColumn) == ans);
  }
}