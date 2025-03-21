#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> shortestDistanceAfterQueries(int n, vector<vector<int>> &queries) {
    vector<vector<int>> prev(n);
    vector<int> dp(n);
    for (int i = 1; i < n; i++) {
      prev[i].push_back(i - 1);
      dp[i] = i;
    }
    vector<int> res;
    for (auto &query : queries) {
      prev[query[1]].push_back(query[0]);
      for (int v = query[1]; v < n; v++) {
        for (int u : prev[v]) {
          dp[v] = min(dp[v], dp[u] + 1);
        }
      }
      res.push_back(dp[n - 1]);
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>>> tests{
      {5, {{2, 4}, {0, 2}, {0, 4}}, {3, 2, 1}},
      {4, {{0, 3}, {0, 2}}, {1, 1}},
  };

  for (auto &[n, queries, ans] : tests) {
    assert(Solution().shortestDistanceAfterQueries(n, queries) == ans);
  }
}