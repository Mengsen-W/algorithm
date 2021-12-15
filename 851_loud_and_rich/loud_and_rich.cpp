/*
 * @Date: 2021-12-15 06:58:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-15 07:50:16
 */

#include <cassert>
#include <numeric>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> loudAndRich(vector<vector<int>> &richer, vector<int> &quiet) {
    int n = quiet.size();
    vector<vector<int>> g(n);
    vector<int> inDeg(n);
    for (auto &r : richer) {
      g[r[0]].emplace_back(r[1]);
      ++inDeg[r[1]];
    }

    vector<int> ans(n);
    iota(ans.begin(), ans.end(), 0);
    queue<int> q;
    for (int i = 0; i < n; ++i) {
      if (inDeg[i] == 0) {
        q.emplace(i);
      }
    }
    while (!q.empty()) {
      int x = q.front();
      q.pop();
      for (int y : g[x]) {
        if (quiet[ans[x]] < quiet[ans[y]]) {
          ans[y] = ans[x];  // 更新 x 的邻居的答案
        }
        if (--inDeg[y] == 0) {
          q.emplace(y);
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<vector<int>> richer = {{1, 0}, {2, 1}, {3, 1}, {3, 7},
                                  {4, 3}, {5, 3}, {6, 3}};
    vector<int> quiet = {3, 2, 5, 4, 6, 1, 7, 0};
    vector<int> ans = {5, 5, 2, 5, 4, 5, 6, 7};
    assert(Solution().loudAndRich(richer, quiet) == ans);
  }
  {
    vector<vector<int>> richer = {};
    vector<int> quiet = {0};
    vector<int> ans = {0};
    assert(Solution().loudAndRich(richer, quiet) == ans);
  }
}