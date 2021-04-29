/*
 * @Date: 2021-04-29 09:30:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-29 09:33:07
 * @FilePath: \algorithm\403_can_cross\can_cross.cpp
 * @Description: file content
 */

#include <cassert>
#include <vector>
using namespace std;

bool can_cross(vector<int>& stones) {
  int n = stones.size();
  vector<vector<int>> dp(n, vector<int>(n));
  dp[0][0] = true;
  for (int i = 1; i < n; ++i) {
    if (stones[i] - stones[i - 1] > i) {
      return false;
    }
  }
  for (int i = 1; i < n; ++i) {
    for (int j = i - 1; j >= 0; --j) {
      int k = stones[i] - stones[j];
      if (k > j + 1) {
        break;
      }
      dp[i][k] = dp[j][k - 1] || dp[j][k] || dp[j][k + 1];
      if (i == n - 1 && dp[i][k]) {
        return true;
      }
    }
  }
  return false;
}

int main() {
  {
    vector<int> stones{0, 1, 3, 5, 6, 8, 12, 17};
    assert(can_cross(stones));
  }
  {
    vector<int> stones{0, 1, 2, 3, 4, 8, 9, 11};
    assert(!can_cross(stones));
  }
}