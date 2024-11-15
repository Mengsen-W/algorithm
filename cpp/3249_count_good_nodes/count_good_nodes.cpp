#include <cassert>
#include <functional>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countGoodNodes(vector<vector<int>>& edges) {
    int n = edges.size() + 1;
    int res = 0;
    vector<vector<int>> g(n);
    for (const auto& edge : edges) {
      g[edge[0]].push_back(edge[1]);
      g[edge[1]].push_back(edge[0]);
    }

    function<int(int, int)> dfs = [&](int node, int parent) -> int {
      bool valid = true;
      int treeSize = 0;
      int subTreeSize = 0;

      for (int child : g[node]) {
        if (child != parent) {
          int size = dfs(child, node);
          if (subTreeSize == 0) {
            subTreeSize = size;
          } else if (size != subTreeSize) {
            valid = false;
          }
          treeSize += size;
        }
      }
      if (valid) {
        res++;
      }
      return treeSize + 1;
    };

    dfs(0, -1);
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{0, 1}, {0, 2}, {1, 3}, {1, 4}, {2, 5}, {2, 6}}, 7},
      {{{0, 1}, {1, 2}, {2, 3}, {3, 4}, {0, 5}, {1, 6}, {2, 7}, {3, 8}}, 6},
      {{{0, 1}, {1, 2}, {1, 3}, {1, 4}, {0, 5}, {5, 6}, {6, 7}, {7, 8}, {0, 9}, {9, 10}, {9, 12}, {10, 11}}, 12},
  };

  for (auto &[edges,ans] : tests) {
    assert(Solution().countGoodNodes(edges) == ans);
  }
}