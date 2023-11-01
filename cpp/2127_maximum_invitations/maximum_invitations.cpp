/*
 * @Date: 2023-11-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-01
 * @FilePath: /algorithm/cpp/2127_maximum_invitations/maximum_invitations.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumInvitations(vector<int>& favorite) {
    int n = favorite.size();
    // 统计入度，便于进行拓扑排序
    vector<int> indeg(n);
    for (int i = 0; i < n; ++i) {
      ++indeg[favorite[i]];
    }
    vector<int> used(n), f(n, 1);
    queue<int> q;
    for (int i = 0; i < n; ++i) {
      if (!indeg[i]) {
        q.push(i);
      }
    }
    while (!q.empty()) {
      int u = q.front();
      used[u] = true;
      q.pop();
      int v = favorite[u];
      // 状态转移
      f[v] = max(f[v], f[u] + 1);
      --indeg[v];
      if (!indeg[v]) {
        q.push(v);
      }
    }
    int ring = 0, total = 0;
    for (int i = 0; i < n; ++i) {
      if (!used[i]) {
        int j = favorite[i];
        if (favorite[j] == i) {
          total += f[i] + f[j];
          used[i] = used[j] = true;
        } else {
          int u = i, cnt = 0;
          while (true) {
            ++cnt;
            u = favorite[u];
            used[u] = true;
            if (u == i) {
              break;
            }
          }
          ring = max(ring, cnt);
        }
      }
    }
    return max(ring, total);
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 2, 1, 2}, 3},
      {{1, 2, 0}, 3},
      {{3, 0, 1, 4, 1}, 4},
  };

  for (auto& [favorite, ans] : tests) {
    assert(Solution().maximumInvitations(favorite) == ans);
  }
}