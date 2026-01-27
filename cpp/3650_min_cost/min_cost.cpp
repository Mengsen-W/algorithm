#include <cassert>
#include <climits>
#include <queue>
#include <tuple>
#include <utility>
#include <vector>

using namespace std;

class Solution {
  using PII = pair<int, int>;

 public:
  int minCost(int n, vector<vector<int>> &edges) {
    vector<vector<PII>> g(n);
    for (auto &e : edges) {
      int x = e[0], y = e[1], w = e[2];
      g[x].emplace_back(y, w);
      g[y].emplace_back(x, 2 * w);
    }

    vector<int> d(n, INT_MAX);
    vector<bool> v(n, false);
    priority_queue<PII, vector<PII>, greater<PII>> q;
    d[0] = 0;
    q.emplace(0, 0);

    while (!q.empty()) {
      int x = q.top().second;
      q.pop();
      if (x == n - 1) {
        return d[x];
      }
      // 只有第一次出堆需要去松弛其他点
      if (v[x]) {
        continue;
      }
      v[x] = 1;

      for (auto &[y, w] : g[x]) {
        if (d[x] + w < d[y]) {
          d[y] = d[x] + w;
          q.emplace(d[y], y);
        }
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {4, {{0, 1, 3}, {3, 1, 1}, {2, 3, 4}, {0, 2, 2}}, 5},
      {4, {{0, 2, 1}, {2, 1, 1}, {1, 3, 1}, {2, 3, 3}}, 3},
  };

  for (auto &[n, edges, ans] : tests) {
    assert(Solution().minCost(n, edges) == ans);
  }
}
