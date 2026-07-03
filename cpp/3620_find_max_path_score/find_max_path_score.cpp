#include <cassert>
#include <climits>
#include <queue>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int findMaxPathScore(vector<vector<int>>& edges, vector<bool>& online, long long k) {
    int n = online.size();
    vector<vector<pair<int, int>>> g(n);
    int l = INT_MAX, r = 0;

    for (auto& edge : edges) {
      int u = edge[0];
      int v = edge[1];
      int w = edge[2];
      if (!online[u] || !online[v]) {
        continue;
      }
      g[u].push_back({v, w});
      l = min(l, w);
      r = max(r, w);
    }

    auto check = [&](int mid) -> bool {
      vector<long long> dis(n, LLONG_MAX);
      priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> q;

      dis[0] = 0;
      q.push({0, 0});

      while (!q.empty()) {
        auto [d, u] = q.top();
        q.pop();

        if (d > k) {
          return false;
        }
        if (u == n - 1) {
          return true;
        }
        if (d > dis[u]) {
          continue;
        }

        for (auto& [v, w] : g[u]) {
          if (w < mid) {
            continue;
          }
          if (dis[v] > dis[u] + w) {
            dis[v] = dis[u] + w;
            q.push({dis[v], v});
          }
        }
      }
      return false;
    };

    if (!check(l)) {
      return -1;
    }

    while (l <= r) {
      int mid = (l + r) >> 1;
      if (check(mid)) {
        l = mid + 1;
      } else {
        r = mid - 1;
      }
    }
    return r;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<bool>, long long, int>> tests{
      {{{0, 1, 5}, {1, 3, 10}, {0, 2, 3}, {2, 3, 4}}, {true, true, true, true}, 10, 3},
      {{{0, 1, 7}, {1, 4, 5}, {0, 2, 6}, {2, 3, 6}, {3, 4, 2}, {2, 4, 6}}, {true, true, true, false, true}, 12, 6},
  };

  for (auto& [edges, online, k, ans] : tests) {
    assert(Solution().findMaxPathScore(edges, online, k) == ans);
  }
}