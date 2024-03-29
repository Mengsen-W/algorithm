/*
 * @Date: 2022-01-24 09:19:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-24 09:27:42
 */

#include <cassert>
#include <climits>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  int secondMinimum(int n, vector<vector<int>> edges, int time, int change) {
    vector<vector<int>> graph(n + 1);
    for (auto& e : edges) {
      graph[e[0]].push_back(e[1]);
      graph[e[1]].push_back(e[0]);
    }

    // path[i][0] 表示从 1 到 i 的最短路长度，path[i][1] 表示从 1 到 i
    // 的严格次短路长度
    vector<vector<int>> path(n + 1, vector<int>(2, INT_MAX));
    path[1][0] = 0;
    queue<pair<int, int>> q;
    q.push({1, 0});
    while (path[n][1] == INT_MAX) {
      auto [cur, len] = q.front();
      q.pop();
      for (auto next : graph[cur]) {
        if (len + 1 < path[next][0]) {
          path[next][0] = len + 1;
          q.push({next, len + 1});
        } else if (len + 1 > path[next][0] && len + 1 < path[next][1]) {
          path[next][1] = len + 1;
          q.push({next, len + 1});
        }
      }
    }

    int ret = 0;
    for (int i = 0; i < path[n][1]; i++) {
      if (ret % (2 * change) >= change) {
        ret = ret + (2 * change - ret % (2 * change));
      }
      ret = ret + time;
    }
    return ret;
  }
};

int main() {
  assert(Solution().secondMinimum(5, {{1, 2}, {1, 3}, {1, 4}, {3, 4}, {4, 5}},
                                  3, 5) == 13);
  assert(Solution().secondMinimum(2, {{1, 2}}, 3, 2) == 11);
}