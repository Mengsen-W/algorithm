#include <bitset>
#include <cassert>
#include <numeric>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

constexpr int MAXN = 100005;

class Solution {
 public:
  vector<int> remainingMethods(int n, int k, vector<vector<int>>& invocations) {
    vector<vector<int>> edges(n);
    vector<int> inDegree(n, 0);

    bitset<MAXN> suspicious;

    for (const auto& inv : invocations) {
      edges[inv[0]].push_back(inv[1]);
      inDegree[inv[1]]++;
    }

    queue<int> q;
    q.push(k);

    suspicious.set(k);

    while (!q.empty()) {
      int u = q.front();
      q.pop();
      for (int v : edges[u]) {
        inDegree[v]--;

        if (!suspicious.test(v)) {
          q.push(v);
          suspicious.set(v);
        }
      }
    }

    bool canRemoveAll = true;
    vector<int> remaining;

    for (int i = 0; i < n; i++) {
      if (suspicious.test(i) && inDegree[i] > 0) {
        canRemoveAll = false;
        break;
      } else if (!suspicious.test(i)) {
        remaining.push_back(i);
      }
    }

    if (!canRemoveAll) {
      vector<int> allNodes(n);
      iota(allNodes.begin(), allNodes.end(), 0);
      return allNodes;
    }

    return remaining;
  }
};

int main() {
  vector<tuple<int, int, vector<vector<int>>, vector<int>>> tests{
      {4, 1, {{1, 2}, {0, 1}, {3, 2}}, {0, 1, 2, 3}},
      {5, 0, {{1, 2}, {0, 2}, {0, 1}, {3, 4}}, {3, 4}},
      {3, 2, {{1, 2}, {0, 1}, {2, 0}}, {}},
  };

  for (auto& [n, k, invocations, ans] : tests) {
    assert(Solution().remainingMethods(n, k, invocations) == ans);
  }
  return 0;
}