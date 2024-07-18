#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> minimumTime(int n, vector<vector<int>>& edges, vector<int>& disappear) {
    vector<vector<pair<int, int>>> adj(n);
    for (const auto& edge : edges) {
      int u = edge[0], v = edge[1], length = edge[2];
      adj[u].emplace_back(v, length);
      adj[v].emplace_back(u, length);
    }
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> pq;
    pq.emplace(0, 0);
    vector<int> answer(n, -1);
    answer[0] = 0;
    while (!pq.empty()) {
      auto [t, u] = pq.top();
      pq.pop();
      if (t != answer[u]) {
        continue;
      }
      for (const auto& [v, length] : adj[u]) {
        if (t + length < disappear[v] && (answer[v] == -1 || t + length < answer[v])) {
          pq.emplace(t + length, v);
          answer[v] = t + length;
        }
      }
    }
    return answer;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>, vector<int>>> tests{
      {3, {{0, 1, 2}, {1, 2, 1}, {0, 2, 4}}, {1, 1, 5}, {0, -1, 4}},
      {3, {{0, 1, 2}, {1, 2, 1}, {0, 2, 4}}, {1, 3, 5}, {0, 2, 3}},
      {2, {{0, 1, 1}}, {1, 1}, {0, -1}},
  };

  for (auto& [n, edges, disappear, ans] : tests) {
    assert(Solution().minimumTime(n, edges, disappear) == ans);
  }
}