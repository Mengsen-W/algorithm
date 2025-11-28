#include <cassert>
#include <functional>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxKDivisibleComponents(int n, vector<vector<int>>& edges, vector<int>& values, int k) {
    vector<vector<int>> graph(n);
    for (auto& edge : edges) {
      int u = edge[0], v = edge[1];
      graph[u].push_back(v);
      graph[v].push_back(u);
    }

    int result = 0;
    function<long long(int, int)> dfs = [&](int node, int parent) -> long long {
      long long sum = values[node];
      for (int neighbor : graph[node]) {
        if (neighbor != parent) {
          sum += dfs(neighbor, node);
        }
      }
      if (sum % k == 0) {
        ++result;
      }
      return sum;
    };
    dfs(0, -1);
    return result;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>, int, int>> tests{
      {5, {{0, 2}, {1, 2}, {1, 3}, {2, 4}}, {1, 8, 1, 4, 4}, 6, 2},
      {7, {{0, 1}, {0, 2}, {1, 3}, {1, 4}, {2, 5}, {2, 6}}, {3, 0, 6, 1, 5, 2, 1}, 3, 3},
  };

  for (auto& [n, edges, values, k, ans] : tests) {
    assert(Solution().maxKDivisibleComponents(n, edges, values, k) == ans);
  }
}