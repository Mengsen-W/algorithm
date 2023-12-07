/*
 * @Date: 2023-12-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-07
 * @FilePath: /algorithm/cpp/1466_min_reorder/min_reorder.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int dfs(int x, int parent, vector<vector<pair<int, int>>>& e) {
    int res = 0;
    for (auto& edge : e[x]) {
      if (edge.first == parent) {
        continue;
      }
      res += edge.second + dfs(edge.first, x, e);
    }
    return res;
  }

  int minReorder(int n, vector<vector<int>>& connections) {
    vector<vector<pair<int, int>>> e(n);
    for (auto edge : connections) {
      e[edge[0]].push_back(make_pair(edge[1], 1));
      e[edge[1]].push_back(make_pair(edge[0], 0));
    }
    return dfs(0, -1, e);
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {6, {{0, 1}, {1, 3}, {2, 3}, {4, 0}, {4, 5}}, 3},
      {5, {{1, 0}, {1, 2}, {3, 2}, {3, 4}}, 2},
      {3, {{1, 0}, {2, 0}}, 0},
  };

  for (auto& [n, connections, ans] : tests) {
    assert(Solution().minReorder(n, connections) == ans);
  }
}