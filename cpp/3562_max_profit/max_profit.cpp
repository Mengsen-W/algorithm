#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProfit(int n, vector<int>& present, vector<int>& future, vector<vector<int>>& hierarchy, int budget) {
    vector<vector<int>> g(n);

    for (auto& e : hierarchy) {
      g[e[0] - 1].push_back(e[1] - 1);
    }

    auto dfs = [&](auto&& self, int u) -> tuple<vector<int>, vector<int>, int> {
      int cost = present[u];
      int dCost = present[u] / 2;  // discounted cost

      // dp[u][state][budget]
      // state = 0: 不购买父节点, state = 1: 必须购买父节点
      auto dp0 = vector(budget + 1, 0);
      auto dp1 = vector(budget + 1, 0);

      // subProfit[state][budget]
      // state = 0: 优惠不可用, state = 1: 优惠可用
      auto subProfit0 = vector(budget + 1, 0);
      auto subProfit1 = vector(budget + 1, 0);

      int uSize = cost;

      for (auto v : g[u]) {
        auto [subDp0, subDp1, vSize] = self(self, v);
        uSize += vSize;
        for (int i = budget; i >= 0; i--) {
          for (int sub = 0; sub <= min(vSize, i); sub++) {
            subProfit0[i] = max(subProfit0[i], subProfit0[i - sub] + subDp0[sub]);
            subProfit1[i] = max(subProfit1[i], subProfit1[i - sub] + subDp1[sub]);
          }
        }
      }

      for (int i = 0; i <= budget; i++) {
        dp0[i] = dp1[i] = subProfit0[i];

        if (i >= dCost) {
          dp1[i] = max(subProfit0[i], subProfit1[i - dCost] + future[u] - dCost);
        }

        if (i >= cost) {
          dp0[i] = max(subProfit0[i], subProfit1[i - cost] + future[u] - cost);
        }
      }

      return {dp0, dp1, uSize};
    };

    return std::get<0>(dfs(dfs, 0))[budget];
  }
};

int main() {
  vector<tuple<int, vector<int>, vector<int>, vector<vector<int>>, int, int>> tests{
      {2, {1, 2}, {4, 3}, {{1, 2}}, 3, 5},
      {2, {3, 4}, {5, 8}, {{1, 2}}, 4, 4},
      {3, {4, 6, 8}, {7, 9, 11}, {{1, 2}, {1, 3}}, 10, 10},
      {3, {5, 2, 3}, {8, 5, 6}, {{1, 2}, {2, 3}}, 7, 12},
  };

  for (auto& [n, present, future, hierarchy, budget, expect] : tests) {
    assert(Solution().maxProfit(n, present, future, hierarchy, budget) == expect);
  }
}