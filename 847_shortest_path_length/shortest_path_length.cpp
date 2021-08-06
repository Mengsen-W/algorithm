/*
 * @Date: 2021-08-06 10:54:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-06 11:39:13
 */

#include <cassert>
#include <climits>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int shortestPathLength_bfs(vector<vector<int>>& graph) {
    int n = graph.size();
    queue<tuple<int, int, int>> q;
    vector<vector<int>> seen(n, vector<int>(1 << n));
    for (int i = 0; i < n; ++i) {
      q.emplace(i, 1 << i, 0);
      seen[i][1 << i] = true;
    }

    int ans = 0;
    while (!q.empty()) {
      auto [u, mask, dist] = q.front();
      q.pop();
      if (mask == (1 << n) - 1) {
        ans = dist;
        break;
      }
      // 搜索相邻的节点
      for (int v : graph[u]) {
        // 将 mask 的第 v 位置为 1
        int mask_v = mask | (1 << v);
        if (!seen[v][mask_v]) {
          q.emplace(v, mask_v, dist + 1);
          seen[v][mask_v] = true;
        }
      }
    }
    return ans;
  }

  int shortestPathLength_floyd(vector<vector<int>>& graph) {
    int n = graph.size();
    vector<vector<int>> d(n, vector<int>(n, n + 1));
    for (int i = 0; i < n; ++i) {
      for (int j : graph[i]) {
        d[i][j] = 1;
      }
    }
    // 使用 floyd 算法预处理出所有点对之间的最短路径长度
    for (int k = 0; k < n; ++k) {
      for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
          d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
        }
      }
    }

    vector<vector<int>> f(n, vector<int>(1 << n, INT_MAX / 2));
    for (int mask = 1; mask < (1 << n); ++mask) {
      // 如果 mask 只包含一个 1，即 mask 是 2 的幂
      if ((mask & (mask - 1)) == 0) {
        int u = __builtin_ctz(mask);
        f[u][mask] = 0;
      } else {
        for (int u = 0; u < n; ++u) {
          if (mask & (1 << u)) {
            for (int v = 0; v < n; ++v) {
              if ((mask & (1 << v)) && u != v) {
                f[u][mask] = min(f[u][mask], f[v][mask ^ (1 << u)] + d[v][u]);
              }
            }
          }
        }
      }
    }

    int ans = INT_MAX;
    for (int u = 0; u < n; ++u) {
      ans = min(ans, f[u][(1 << n) - 1]);
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> graph = {{1, 2, 3}, {0}, {0}, {0}};
    assert(Solution{}.shortestPathLength_bfs(graph) == 4);
    assert(Solution{}.shortestPathLength_floyd(graph) == 4);
  }
  {
    vector<vector<int>> graph = {{1}, {0, 2, 4}, {1, 3, 4}, {2}, {1, 2}};
    assert(Solution{}.shortestPathLength_bfs(graph) == 4);
    assert(Solution{}.shortestPathLength_floyd(graph) == 4);
  }

  return 0;
}