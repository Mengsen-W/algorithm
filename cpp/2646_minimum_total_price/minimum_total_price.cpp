/*
 * @Date: 2023-12-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-06
 * @FilePath: /algorithm/cpp/2646_minimum_total_price/minimum_total_price.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int find(vector<int> &uf, int i) {
    if (uf[i] == i) {
      return i;
    }
    uf[i] = find(uf, uf[i]);
    return uf[i];
  }

  int minimumTotalPrice(int n, vector<vector<int>> &edges, vector<int> &price, vector<vector<int>> &trips) {
    vector<vector<int>> next(n);
    for (auto &edge : edges) {
      next[edge[0]].push_back(edge[1]);
      next[edge[1]].push_back(edge[0]);
    }

    vector<vector<int>> query(n);
    for (auto &trip : trips) {
      query[trip[0]].push_back(trip[1]);
      if (trip[0] != trip[1]) {
        query[trip[1]].push_back(trip[0]);
      }
    }

    vector<int> uf(n), visited(n), diff(n), parent(n);
    function<void(int, int)> tarjan = [&](int node, int p) {
      parent[node] = p;
      uf[node] = node;
      for (int child : next[node]) {
        if (child == p) {
          continue;
        }
        tarjan(child, node);
        uf[child] = node;
      }
      for (int node1 : query[node]) {
        if (node != node1 && !visited[node1]) {
          continue;
        }
        int lca = find(uf, node1);
        diff[node]++;
        diff[node1]++;
        diff[lca]--;
        if (parent[lca] >= 0) {
          diff[parent[lca]]--;
        }
      }
      visited[node] = 1;
    };
    tarjan(0, -1);

    vector<int> count(n);
    function<int(int, int)> dfs = [&](int node, int p) -> int {
      count[node] = diff[node];
      for (int child : next[node]) {
        if (child == p) {
          continue;
        }
        count[node] += dfs(child, node);
      }
      return count[node];
    };
    dfs(0, -1);

    function<pair<int, int>(int, int)> dp = [&](int node, int p) -> pair<int, int> {
      pair<int, int> res = {price[node] * count[node], price[node] * count[node] / 2};
      for (int child : next[node]) {
        if (child == p) {
          continue;
        }
        auto [x, y] = dp(child, node);
        res.first += min(x, y);  // node 没有减半，因此可以取子树的两种情况的最小值
        res.second += x;         // node 减半，只能取子树没有减半的情况
      }
      return res;
    };
    auto [x, y] = dp(0, -1);
    return min(x, y);
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>, vector<vector<int>>, int>> tests{
      {4, {{0, 1}, {1, 2}, {1, 3}}, {2, 2, 10, 6}, {{0, 3}, {2, 1}, {2, 3}}, 23},
      {2, {{0, 1}}, {2, 2}, {{0, 0}}, 1},
  };

  for (auto &[n, edges, price, trips, ans] : tests) {
    assert(Solution().minimumTotalPrice(n, edges, price, trips) == ans);
  }
}