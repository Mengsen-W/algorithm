#include <cassert>
#include <climits>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minScore(int n, vector<vector<int>>& roads) {
    struct Edge {
      int v, dis;
      bool operator>(const Edge& other) const { return dis > other.dis; }
    };

    vector<vector<Edge>> graph(n + 1);
    vector<bool> vis(n + 1, false);

    int ans = INT_MAX;
    priority_queue<Edge, vector<Edge>, greater<Edge>> pq;

    for (const auto& road : roads) {
      int u = road[0], v = road[1], dis = road[2];
      graph[u].push_back({v, dis});
      graph[v].push_back({u, dis});

      if (pq.empty() && (u == 1 || v == 1)) {
        pq.push({v, dis});
      }
    }

    while (!pq.empty()) {
      auto [u, dis] = pq.top();
      pq.pop();

      if (vis[u]) {
        continue;
      }

      ans = min(ans, dis);
      vis[u] = true;

      for (const auto& edge : graph[u]) {
        if (!vis[edge.v]) {
          pq.push(edge);
        }
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {4, {{1, 2, 9}, {2, 3, 6}, {2, 4, 5}}, 5},
      {4, {{1, 2, 2}, {1, 3, 4}, {3, 4, 7}}, 2},
  };

  for (auto& [n, roads, expected] : tests) {
    assert(Solution().minScore(n, roads) == expected);
  }
}