/*
 * @Date: 2022-02-16 03:04:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-16 08:59:19
 */

#include <cassert>
#include <climits>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int checkWays(vector<vector<int>> pairs) {
    unordered_map<int, unordered_set<int>> adj;
    for (auto &p : pairs) {
      adj[p[0]].emplace(p[1]);
      adj[p[1]].emplace(p[0]);
    }
    /* 检测是否存在根节点*/
    int root = -1;
    for (auto &[node, neighbours] : adj) {
      if (neighbours.size() == adj.size() - 1) {
        root = node;
        break;
      }
    }
    if (root == -1) {
      return 0;
    }

    int res = 1;
    for (auto &[node, neighbours] : adj) {
      if (node == root) {
        continue;
      }
      int currDegree = neighbours.size();
      int parent = -1;
      int parentDegree = INT_MAX;

      /* 根据 degree 的大小找到 node 的父节点 parent */
      for (auto &neighbour : neighbours) {
        if (adj[neighbour].size() < parentDegree &&
            adj[neighbour].size() >= currDegree) {
          parent = neighbour;
          parentDegree = adj[neighbour].size();
        }
      }
      if (parent == -1) {
        return 0;
      }

      /* 检测 neighbours 是否是 adj[parent] 的子集 */
      for (auto &neighbour : neighbours) {
        if (neighbour == parent) {
          continue;
        }
        if (!adj[parent].count(neighbour)) {
          return 0;
        }
      }
      if (parentDegree == currDegree) {
        res = 2;
      }
    }
    return res;
  }
};

int main() {
  assert(Solution().checkWays({{1, 2}, {2, 3}}) == 1);
  assert(Solution().checkWays({{1, 2}, {2, 3}, {1, 3}}) == 2);
  assert(Solution().checkWays({{1, 2}, {2, 3}, {2, 4}, {1, 5}}) == 0);
  assert(Solution().checkWays({{1, 2}}) == 2);
}