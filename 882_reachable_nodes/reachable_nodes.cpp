/*
 * @Date: 2022-11-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-26
 * @FilePath: /algorithm/882_reachable_nodes/reachable_nodes.cpp
 */

#include <cassert>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int encode(int u, int v, int n) { return u * n + v; }

  int reachableNodes(vector<vector<int>> &edges, int maxMoves, int n) {
    vector<vector<pair<int, int>>> adList(n);
    for (auto &edge : edges) {
      int u = edge[0], v = edge[1], nodes = edge[2];
      adList[u].emplace_back(v, nodes);
      adList[v].emplace_back(u, nodes);
    }

    unordered_map<int, int> used;
    unordered_set<int> visited;
    int reachableNodes = 0;
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> pq;
    pq.emplace(0, 0);
    while (!pq.empty() && pq.top().first <= maxMoves) {
      auto [step, u] = pq.top();
      pq.pop();
      if (visited.count(u)) {
        continue;
      }
      visited.emplace(u);
      reachableNodes++;
      for (auto [v, nodes] : adList[u]) {
        if (nodes + step + 1 <= maxMoves && !visited.count(v)) {
          pq.emplace(nodes + step + 1, v);
        }
        used[encode(u, v, n)] = min(nodes, maxMoves - step);
      }
    }

    for (auto &edge : edges) {
      int u = edge[0], v = edge[1], nodes = edge[2];
      reachableNodes += min(nodes, used[encode(u, v, n)] + used[encode(v, u, n)]);
    }
    return reachableNodes;
  }
};

int main() {
  {
    vector<vector<int>> edges{{0, 1, 10}, {0, 2, 1}, {1, 2, 2}};
    int maxMoves = 6;
    int n = 3;
    int ans = 13;
    assert(Solution().reachableNodes(edges, maxMoves, n) == ans);
  }

  {
    vector<vector<int>> edges{{0, 1, 4}, {1, 2, 6}, {0, 2, 8}, {1, 3, 1}};
    int maxMoves = 10;
    int n = 4;
    int ans = 23;
    assert(Solution().reachableNodes(edges, maxMoves, n) == ans);
  }

  {
    vector<vector<int>> edges{{1, 2, 4}, {1, 4, 5}, {1, 3, 1}, {2, 3, 4}, {3, 4, 5}};
    int maxMoves = 17;
    int n = 5;
    int ans = 1;
    assert(Solution().reachableNodes(edges, maxMoves, n) == ans);
  }
}