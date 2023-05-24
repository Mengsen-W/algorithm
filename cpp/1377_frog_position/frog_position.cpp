/*
 * @Date: 2023-05-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-24
 * @FilePath: /algorithm/cpp/1377_frog_position/frog_position.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  double frogPosition(int n, vector<vector<int>>& edges, int t, int target) {
    vector<vector<int>> G(n + 1);
    for (int i = 0; i < edges.size(); ++i) {
      G[edges[i][0]].push_back(edges[i][1]);
      G[edges[i][1]].push_back(edges[i][0]);
    }
    vector<bool> visited(n + 1, false);
    return dfs(G, visited, 1, t, target);
  }

  double dfs(vector<vector<int>>& G, vector<bool>& visited, int i, int t, int target) {
    int nxt = i == 1 ? G[i].size() : G[i].size() - 1;
    if (t == 0 || nxt == 0) {
      return i == target ? 1.0 : 0.0;
    }
    visited[i] = true;
    double ans = 0.0;
    for (int j : G[i]) {
      if (!visited[j]) {
        ans += dfs(G, visited, j, t - 1, target);
      }
    }
    return ans / nxt;
  }
};

int main() {
  {
    int n = 7;
    vector<vector<int>> edges{{1, 2}, {1, 3}, {1, 7}, {2, 4}, {2, 6}, {3, 5}};
    int t = 2;
    int target = 4;
    double ans = 0.16666666666666666;
    assert(Solution().frogPosition(n, edges, t, target) == ans);
  }

  {
    int n = 7;
    vector<vector<int>> edges{{1, 2}, {1, 3}, {1, 7}, {2, 4}, {2, 6}, {3, 5}};
    int t = 1;
    int target = 7;
    double ans = 0.3333333333333333;
    assert(Solution().frogPosition(n, edges, t, target) == ans);
  }
}