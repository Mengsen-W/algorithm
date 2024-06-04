#include <cassert>
#include <functional>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> countPairsOfConnectableServers(vector<vector<int>> &edges, int signalSpeed) {
    int n = edges.size() + 1;
    vector<vector<pair<int, int>>> graph(n);

    for (auto e : edges) {
      graph[e[0]].emplace_back(e[1], e[2]);
      graph[e[1]].emplace_back(e[0], e[2]);
    }
    function<int(int, int, int)> dfs = [&](int p, int root, int curr) -> int {
      int res = 0;
      if (curr == 0) {
        res++;
      }
      for (auto &[v, cost] : graph[p]) {
        if (v != root) {
          res += dfs(v, p, (curr + cost) % signalSpeed);
        }
      }
      return res;
    };

    vector<int> res(n);
    for (int i = 0; i < n; i++) {
      int pre = 0;
      for (auto &[v, cost] : graph[i]) {
        int cnt = dfs(v, i, cost % signalSpeed);
        res[i] += pre * cnt;
        pre += cnt;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, vector<int>>> tests{
      {{{0, 1, 1}, {1, 2, 5}, {2, 3, 13}, {3, 4, 9}, {4, 5, 2}}, 1, {0, 4, 6, 6, 4, 0}},
      {{{0, 6, 3}, {6, 5, 3}, {0, 3, 1}, {3, 2, 7}, {3, 1, 6}, {3, 4, 2}}, 3, {2, 0, 0, 0, 0, 0, 2}},
  };

  for (auto &[edges, signalSpeed, ans] : tests) {
    assert(Solution().countPairsOfConnectableServers(edges, signalSpeed) == ans);
  }
}
