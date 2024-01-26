/*
 * @Date: 2024-01-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-26
 * @FilePath: /algorithm/cpp/2846_min_operations_queries/min_operations_queries.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

const int W = 26;

class Solution {
 public:
  int find(vector<int>& uf, int i) {
    if (uf[i] == i) {
      return i;
    }
    uf[i] = find(uf, uf[i]);
    return uf[i];
  }

  vector<int> minOperationsQueries(int n, vector<vector<int>>& edges, vector<vector<int>>& queries) {
    int m = queries.size();
    vector<unordered_map<int, int>> neighbors(n);
    for (auto& edge : edges) {
      neighbors[edge[0]][edge[1]] = edge[2];
      neighbors[edge[1]][edge[0]] = edge[2];
    }
    vector<vector<pair<int, int>>> queryArr(n);
    for (int i = 0; i < m; i++) {
      queryArr[queries[i][0]].push_back({queries[i][1], i});
      queryArr[queries[i][1]].push_back({queries[i][0], i});
    }

    vector<vector<int>> count(n, vector<int>(W + 1));
    vector<int> visited(n), uf(n), lca(m);
    function<void(int, int)> tarjan = [&](int node, int parent) {
      if (parent != -1) {
        count[node] = count[parent];
        count[node][neighbors[node][parent]]++;
      }
      uf[node] = node;
      for (auto [child, _] : neighbors[node]) {
        if (child == parent) {
          continue;
        }
        tarjan(child, node);
        uf[child] = node;
      }
      for (auto [node1, index] : queryArr[node]) {
        if (node != node1 && !visited[node1]) {
          continue;
        }
        lca[index] = find(uf, node1);
      }
      visited[node] = 1;
    };
    tarjan(0, -1);
    vector<int> res(m);
    for (int i = 0; i < m; i++) {
      int totalCount = 0, maxCount = 0;
      for (int j = 1; j <= W; j++) {
        int t = count[queries[i][0]][j] + count[queries[i][1]][j] - 2 * count[lca[i]][j];
        maxCount = max(maxCount, t);
        totalCount += t;
      }
      res[i] = totalCount - maxCount;
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<vector<int>>, vector<int>>> tests{
      {
          7,
          {{0, 1, 1}, {1, 2, 1}, {2, 3, 1}, {3, 4, 2}, {4, 5, 2}, {5, 6, 2}},
          {{0, 3}, {3, 6}, {2, 6}, {0, 6}},
          {0, 0, 1, 3},
      },
      {
          8,
          {{1, 2, 6}, {1, 3, 4}, {2, 4, 6}, {2, 5, 3}, {3, 6, 6}, {3, 0, 8}, {7, 0, 2}},
          {{4, 6}, {0, 4}, {6, 5}, {7, 4}},
          {1, 2, 2, 3},
      },
  };

  for (auto& [n, edges, queries, ans] : tests) {
    assert(Solution().minOperationsQueries(n, edges, queries) == ans);
  }
}
