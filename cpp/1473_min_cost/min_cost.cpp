/*
 * @Date: 2021-05-06 10:30:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-06 10:46:52
 */

#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

// 极大值
// 选择 INT_MAX / 2 的原因是防止整数相加溢出
static constexpr int INFTY = INT_MAX / 2;

using TIII = tuple<int, int, int>;

int min_cost(vector<int>& houses, vector<vector<int>>& cost, int m, int n,
             int target) {
  // 将颜色调整为从 0 开始编号，没有被涂色标记为 -1
  for (int& c : houses) {
    --c;
  }

  // dp 所有元素初始化为极大值
  vector<vector<vector<int>>> dp(
      m, vector<vector<int>>(n, vector<int>(target, INFTY)));
  vector<vector<TIII>> best(m, vector<TIII>(target, TIII{INFTY, -1, INFTY}));

  for (int i = 0; i < m; ++i) {
    for (int j = 0; j < n; ++j) {
      if (houses[i] != -1 && houses[i] != j) {
        continue;
      }

      for (int k = 0; k < target; ++k) {
        if (i == 0) {
          if (k == 0) {
            dp[i][j][k] = 0;
          }
        } else {
          dp[i][j][k] = dp[i - 1][j][k];
          if (k > 0) {
            // 使用 best(i-1,k-1) 直接得到 dp(i,j,k) 的值
            auto&& [first, first_idx, second] = best[i - 1][k - 1];
            dp[i][j][k] = min(dp[i][j][k], (j == first_idx ? second : first));
          }
        }

        if (dp[i][j][k] != INFTY && houses[i] == -1) {
          dp[i][j][k] += cost[i][j];
        }

        // 用 dp(i,j,k) 更新 best(i,k)
        auto&& [first, first_idx, second] = best[i][k];
        if (dp[i][j][k] < first) {
          second = first;
          first = dp[i][j][k];
          first_idx = j;
        } else if (dp[i][j][k] < second) {
          second = dp[i][j][k];
        }
      }
    }
  }

  int ans = INFTY;
  for (int j = 0; j < n; ++j) {
    ans = min(ans, dp[m - 1][j][target - 1]);
  }
  return ans == INFTY ? -1 : ans;
}

int main() {
  {
    vector<int> houses{0, 0, 0, 0, 0};
    vector<vector<int>> cost = {{1, 10}, {10, 1}, {10, 1}, {1, 10}, {5, 1}};
    int m = 5;
    int n = 2;
    int target = 3;
    assert(min_cost(houses, cost, m, n, target) == 9);
  }
  {
    vector<int> houses{0, 2, 1, 2, 0};
    vector<vector<int>> cost = {{1, 10}, {10, 1}, {10, 1}, {1, 10}, {5, 1}};
    int m = 5;
    int n = 2;
    int target = 3;
    assert(min_cost(houses, cost, m, n, target) == 11);
  }
  {
    vector<int> houses{0, 0, 0, 0, 0};
    vector<vector<int>> cost = {{1, 10}, {10, 1}, {1, 10}, {10, 1}, {1, 10}};
    int m = 5;
    int n = 2;
    int target = 5;
    assert(min_cost(houses, cost, m, n, target) == 5);
  }
  {
    vector<int> houses{3, 1, 2, 3};
    vector<vector<int>> cost = {{1, 1, 1}, {1, 1, 1}, {1, 1, 1}, {1, 1, 1}};
    int m = 4;
    int n = 3;
    int target = 3;
    assert(min_cost(houses, cost, m, n, target) == -1);
  }
  return 0;
}