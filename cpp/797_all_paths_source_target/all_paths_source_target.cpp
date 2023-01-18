/*
 * @Date: 2021-08-25 10:42:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-25 10:49:49
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<vector<int>> ans;
  vector<int> stk;

  void dfs(vector<vector<int>>& graph, int x, int n) {
    if (x == n) {
      ans.push_back(stk);
      return;
    }
    for (auto& y : graph[x]) {
      stk.push_back(y);
      dfs(graph, y, n);
      stk.pop_back();
    }
  }

  vector<vector<int>> allPathsSourceTarget(vector<vector<int>>& graph) {
    stk.push_back(0);
    dfs(graph, 0, graph.size() - 1);
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> graph{{1, 2}, {3}, {3}, {}};
    vector<vector<int>> ans{{0, 1, 3}, {0, 2, 3}};
    assert(Solution().allPathsSourceTarget(graph) == ans);
  }
  {
    vector<vector<int>> graph{{4, 3, 1}, {3, 2, 4}, {3}, {4}, {}};
    vector<vector<int>> ans{
        {0, 4}, {0, 3, 4}, {0, 1, 3, 4}, {0, 1, 2, 3, 4}, {0, 1, 4}};
    assert(Solution().allPathsSourceTarget(graph) == ans);
  }
  {
    vector<vector<int>> graph{{1}, {}};
    vector<vector<int>> ans{{0, 1}};
    assert(Solution().allPathsSourceTarget(graph) == ans);
  }
  {
    vector<vector<int>> graph{{1, 2, 3}, {2}, {3}, {}};
    vector<vector<int>> ans{{0, 1, 2, 3}, {0, 2, 3}, {0, 3}};
    assert(Solution().allPathsSourceTarget(graph) == ans);
  }
  {
    vector<vector<int>> graph{{1, 3}, {2}, {3}, {}};
    vector<vector<int>> ans{{0, 1, 2, 3}, {0, 3}};
    assert(Solution().allPathsSourceTarget(graph) == ans);
  }
}