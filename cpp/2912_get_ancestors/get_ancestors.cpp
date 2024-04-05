/*
 * @Date: 2024-04-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-05
 * @FilePath: /algorithm/cpp/2912_get_ancestors/get_ancestors.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> getAncestors(int n, vector<vector<int>>& edges) {
    vector<unordered_set<int>> anc(n);  // 存储每个节点祖先的辅助数组
    vector<vector<int>> e(n);           // 邻接表
    vector<int> indeg(n);               // 入度表
    // 预处理
    for (const auto& edge : edges) {
      e[edge[0]].push_back(edge[1]);
      ++indeg[edge[1]];
    }
    // 广度优先搜索求解拓扑排序
    queue<int> q;
    for (int i = 0; i < n; ++i) {
      if (!indeg[i]) {
        q.push(i);
      }
    }
    while (!q.empty()) {
      int u = q.front();
      q.pop();
      for (int v : e[u]) {
        // 更新子节点的祖先哈希表
        anc[v].insert(u);
        for (int i : anc[u]) {
          anc[v].insert(i);
        }
        --indeg[v];
        if (!indeg[v]) {
          q.push(v);
        }
      }
    }
    // 转化为答案数组
    vector<vector<int>> res(n);
    for (int i = 0; i < n; ++i) {
      for (int j : anc[i]) {
        res[i].push_back(j);
      }
      sort(res[i].begin(), res[i].end());
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, vector<vector<int>>>> tests{
      {
          8,
          {{{0, 3}, {0, 4}, {1, 3}, {2, 4}, {2, 7}, {3, 5}, {3, 6}, {3, 7}, {4, 6}}},
          {{{}, {}, {}, {0, 1}, {0, 2}, {0, 1, 3}, {0, 1, 2, 3, 4}, {0, 1, 2, 3}}},
      },
      {
          5,
          {{{0, 1}, {0, 2}, {0, 3}, {0, 4}, {1, 2}, {1, 3}, {1, 4}, {2, 3}, {2, 4}, {3, 4}}},
          {{{}, {0}, {0, 1}, {0, 1, 2}, {0, 1, 2, 3}}},
      },
  };

  for (auto& [n, edges, ans] : tests) {
    assert(Solution().getAncestors(n, edges) == ans);
  }
}