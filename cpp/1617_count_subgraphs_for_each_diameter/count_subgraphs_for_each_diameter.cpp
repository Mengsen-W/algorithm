/*
 * @Date: 2023-03-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-12
 * @FilePath: /algorithm/cpp/1617_count_subgraphs_for_each_diameter/count_subgraphs_for_each_diameter.cpp
 */

#include <cassert>
#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> countSubgraphsForEachDiameter(int n, vector<vector<int>>& edges) {
    vector<vector<int>> adj(n);
    vector<vector<int>> dist(n, vector<int>(n, INT_MAX));
    for (auto& edge : edges) {
      int x = edge[0] - 1;
      int y = edge[1] - 1;
      adj[x].emplace_back(y);
      adj[y].emplace_back(x);
      dist[x][y] = dist[y][x] = 1;
    }
    for (int i = 0; i < n; i++) {
      dist[i][i] = 0;
    }
    for (int i = 0; i < n; i++) {
      for (int j = 0; j < n; j++) {
        for (int k = 0; k < n; k++) {
          if (dist[j][i] != INT_MAX && dist[i][k] != INT_MAX) {
            dist[j][k] = min(dist[j][k], dist[j][i] + dist[i][k]);
          }
        }
      }
    }
    function<int(int, int, int, int)> dfs = [&](int u, int parent, int x, int y) -> int {
      if (dist[u][x] > dist[x][y] || dist[u][y] > dist[x][y]) {
        return 1;
      }
      if ((dist[u][y] == dist[x][y] && u < x) || (dist[u][x] == dist[x][y] && u < y)) {
        return 1;
      }
      int ret = 1;
      for (int v : adj[u]) {
        if (v != parent) {
          ret *= dfs(v, u, x, y);
        }
      }
      if (dist[u][x] + dist[u][y] > dist[x][y]) {
        ret++;
      }
      return ret;
    };
    vector<int> ans(n - 1);
    for (int i = 0; i < n - 1; i++) {
      for (int j = i + 1; j < n; j++) {
        ans[dist[i][j] - 1] += dfs(i, -1, i, j);
      }
    }
    return ans;
  }
};

int main() {
  {
    int n = 4;
    vector<vector<int>> edges{{1, 2}, {2, 3}, {2, 4}};
    vector<int> ans{3, 4, 0};
    assert(Solution().countSubgraphsForEachDiameter(n, edges) == ans);
  }

  {
    int n = 2;
    vector<vector<int>> edges{{1, 2}};
    vector<int> ans{1};
    assert(Solution().countSubgraphsForEachDiameter(n, edges) == ans);
  }

  {
    int n = 3;
    vector<vector<int>> edges{{1, 2}, {2, 3}};
    vector<int> ans{2, 1};
    assert(Solution().countSubgraphsForEachDiameter(n, edges) == ans);
  }
}