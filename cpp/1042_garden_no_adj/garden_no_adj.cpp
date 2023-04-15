/*
 * @Date: 2023-04-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-15
 * @FilePath: /algorithm/cpp/1042_garden_no_adj/garden_no_adj.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> gardenNoAdj(int n, vector<vector<int>> &paths) {
    vector<vector<int>> adj(n);
    for (auto &path : paths) {
      adj[path[0] - 1].emplace_back(path[1] - 1);
      adj[path[1] - 1].emplace_back(path[0] - 1);
    }
    vector<int> ans(n);
    for (int i = 0; i < n; i++) {
      vector<bool> colored(5, false);
      for (auto &vertex : adj[i]) {
        colored[ans[vertex]] = true;
      }
      for (int j = 1; j <= 4; j++) {
        if (colored[j] == 0) {
          ans[i] = j;
          break;
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    int n = 3;
    vector<vector<int>> paths{{1, 2}, {2, 3}, {3, 1}};
    vector<int> ans{1, 2, 3};
    assert(Solution().gardenNoAdj(n, paths) == ans);
  }

  {
    int n = 4;
    vector<vector<int>> paths{{1, 2}, {3, 4}};
    vector<int> ans{1, 2, 1, 2};
    assert(Solution().gardenNoAdj(n, paths) == ans);
  }

  {
    int n = 4;
    vector<vector<int>> paths{{1, 2}, {2, 3}, {3, 4}, {4, 1}, {1, 3}, {2, 4}};
    vector<int> ans{1, 2, 3, 4};
    assert(Solution().gardenNoAdj(n, paths) == ans);
  }
}