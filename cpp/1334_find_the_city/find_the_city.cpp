/*
 * @Date: 2023-11-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-14
 * @FilePath: /algorithm/cpp/1334_find_the_city/find_the_city.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findTheCity(int n, vector<vector<int>> &edges, int distanceThreshold) {
    pair<int, int> ans(INT_MAX / 2, -1);
    vector<vector<int>> dis(n, vector<int>(n, INT_MAX / 2));
    vector<vector<int>> vis(n, vector<int>(n, false));
    vector<vector<int>> mp(n, vector<int>(n, INT_MAX / 2));
    for (auto &eg : edges) {
      int from = eg[0], to = eg[1], weight = eg[2];
      mp[from][to] = mp[to][from] = weight;
    }
    for (int i = 0; i < n; ++i) {
      dis[i][i] = 0;
      for (int j = 0; j < n; ++j) {
        int t = -1;
        for (int k = 0; k < n; ++k) {
          if (!vis[i][k] && (t == -1 || dis[i][k] < dis[i][t])) {
            t = k;
          }
        }
        for (int k = 0; k < n; ++k) {
          dis[i][k] = min(dis[i][k], dis[i][t] + mp[t][k]);
        }
        vis[i][t] = true;
      }
    }
    for (int i = 0; i < n; ++i) {
      int cnt = 0;
      for (int j = 0; j < n; ++j) {
        if (dis[i][j] <= distanceThreshold) {
          cnt++;
        }
      }
      if (cnt <= ans.first) {
        ans = {cnt, i};
      }
    }
    return ans.second;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int, int>> tests{
      {4, {{0, 1, 3}, {1, 2, 1}, {1, 3, 4}, {2, 3, 1}}, 4, 3},
      {5, {{0, 1, 2}, {0, 4, 8}, {1, 2, 3}, {1, 4, 2}, {2, 3, 1}, {3, 4, 1}}, 2, 0},
  };

  for (auto &[n, edges, distanceThreshold, ans] : tests) {
    assert(Solution().findTheCity(n, edges, distanceThreshold) == ans);
  }
}