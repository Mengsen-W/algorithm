/*
 * @Date: 2022-04-06 11:08:29
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-20
 * @FilePath: /algorithm/cpp/310_find_min_height_trees/find_min_height_trees.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findMinHeightTrees(int n, vector<vector<int>> edges) {
    if (n == 1) {
      return {0};
    }
    vector<int> degree(n);
    vector<vector<int>> adj(n);
    for (auto& edge : edges) {
      adj[edge[0]].emplace_back(edge[1]);
      adj[edge[1]].emplace_back(edge[0]);
      degree[edge[0]]++;
      degree[edge[1]]++;
    }
    queue<int> qu;
    vector<int> ans;
    for (int i = 0; i < n; i++) {
      if (degree[i] == 1) {
        qu.emplace(i);
      }
    }
    int remainNodes = n;
    while (remainNodes > 2) {
      int sz = qu.size();
      remainNodes -= sz;
      for (int i = 0; i < sz; i++) {
        int curr = qu.front();
        qu.pop();
        for (auto& v : adj[curr]) {
          if (--degree[v] == 1) {
            qu.emplace(v);
          }
        }
      }
    }
    while (!qu.empty()) {
      ans.emplace_back(qu.front());
      qu.pop();
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<int>>> tests{
      {4, {{1, 0}, {1, 2}, {1, 3}}, {1}},
      {6, {{3, 0}, {3, 1}, {3, 2}, {3, 4}, {5, 4}}, {3, 4}},
  };

  for (auto& [n, edges, ans] : tests) {
    assert(Solution().findMinHeightTrees(n, edges) == ans);
  }
}