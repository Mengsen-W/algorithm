/*
 * @Date: 2022-06-25
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-25
 * @FilePath: /algorithm/091_min_cost/min_cost.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minCost(vector<vector<int>> costs) {
    int n = costs.size();
    vector<int> dp(3);
    for (int j = 0; j < 3; j++) {
      dp[j] = costs[0][j];
    }
    for (int i = 1; i < n; i++) {
      vector<int> dpNew(3);
      for (int j = 0; j < 3; j++) {
        dpNew[j] = min(dp[(j + 1) % 3], dp[(j + 2) % 3]) + costs[i][j];
      }
      dp = dpNew;
    }
    return *min_element(dp.begin(), dp.end());
  }
};

int main() {
  assert(Solution().minCost(vector<vector<int>>{{17, 2, 17}, {16, 16, 5}, {14, 3, 19}}) == 10);
  assert(Solution().minCost(vector<vector<int>>{{7, 6, 2}}) == 2);
}