#include <cassert>
#include <functional>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximalPathQuality(vector<int>& values, vector<vector<int>>& edges, int maxTime) {
    int n = values.size();
    vector<vector<pair<int, int>>> g(n);
    for (const auto& edge : edges) {
      g[edge[0]].emplace_back(edge[1], edge[2]);
      g[edge[1]].emplace_back(edge[0], edge[2]);
    }

    vector<int> visited(n);
    visited[0] = true;
    int ans = 0;

    function<void(int, int, int)> dfs = [&](int u, int time, int value) {
      if (u == 0) {
        ans = max(ans, value);
      }
      for (const auto& [v, dist] : g[u]) {
        if (time + dist <= maxTime) {
          if (!visited[v]) {
            visited[v] = true;
            dfs(v, time + dist, value + values[v]);
            visited[v] = false;
          } else {
            dfs(v, time + dist, value);
          }
        }
      }
    };

    dfs(0, 0, values[0]);
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, int, int>> tests{
      {{0, 32, 10, 43}, {{0, 1, 10}, {1, 2, 15}, {0, 3, 10}}, 49, 75},
      {{5, 10, 15, 20}, {{0, 1, 10}, {1, 2, 10}, {0, 3, 10}}, 30, 25},
      {{1, 2, 3, 4}, {{0, 1, 10}, {1, 2, 11}, {2, 3, 12}, {1, 3, 13}}, 50, 7},
      {{0, 1, 2}, {{1, 2, 10}}, 10, 0},
  };

  for (auto &[values, edges, maxTime, ans] : tests) {
    assert(Solution().maximalPathQuality(values, edges, maxTime) == ans);
  }
}