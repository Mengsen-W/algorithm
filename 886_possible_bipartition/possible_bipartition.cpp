/*
 * @Date: 2022-10-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-16
 * @FilePath: /algorithm/886_possible_bipartition/possible_bipartition.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  bool dfs(int curnode, int nowcolor, vector<int>& color, const vector<vector<int>>& g) {
    color[curnode] = nowcolor;
    for (auto& nextnode : g[curnode]) {
      if (color[nextnode] && color[nextnode] == color[curnode]) {
        return false;
      }
      if (!color[nextnode] && !dfs(nextnode, 3 ^ nowcolor, color, g)) {
        return false;
      }
    }
    return true;
  }

  bool possibleBipartition(int n, vector<vector<int>>& dislikes) {
    vector<int> color(n + 1, 0);
    vector<vector<int>> g(n + 1);
    for (auto& p : dislikes) {
      g[p[0]].push_back(p[1]);
      g[p[1]].push_back(p[0]);
    }
    for (int i = 1; i <= n; ++i) {
      if (color[i] == 0 && !dfs(i, 1, color, g)) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  {
    int n = 4;
    vector<vector<int>> dislikes{{1, 2}, {1, 3}, {2, 4}};
    assert(Solution().possibleBipartition(n, dislikes));
  }

  {
    int n = 3;
    vector<vector<int>> dislikes{{1, 2}, {1, 3}, {2, 3}};
    assert(!Solution().possibleBipartition(n, dislikes));
  }

  {
    int n = 5;
    vector<vector<int>> dislikes{{1, 2}, {2, 3}, {3, 4}, {4, 5}, {1, 5}};
    assert(!Solution().possibleBipartition(n, dislikes));
  }
}