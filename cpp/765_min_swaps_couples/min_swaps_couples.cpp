/*
 * @Date: 2023-11-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-11
 * @FilePath: /algorithm/cpp/765_min_swaps_couples/min_swaps_couples.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minSwapsCouples(vector<int>& row) {
    int n = row.size();
    int tot = n / 2;

    vector<vector<int>> graph(tot);
    for (int i = 0; i < n; i += 2) {
      int l = row[i] / 2;
      int r = row[i + 1] / 2;
      if (l != r) {
        graph[l].push_back(r);
        graph[r].push_back(l);
      }
    }
    vector<int> visited(tot, 0);
    int ret = 0;
    for (int i = 0; i < tot; i++) {
      if (visited[i] == 0) {
        queue<int> q;
        visited[i] = 1;
        q.push(i);
        int cnt = 0;

        while (!q.empty()) {
          int x = q.front();
          q.pop();
          cnt += 1;

          for (int y : graph[x]) {
            if (visited[y] == 0) {
              visited[y] = 1;
              q.push(y);
            }
          }
        }
        ret += cnt - 1;
      }
    }
    return ret;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{0, 2, 1, 3}, 1},
      {{3, 2, 0, 1}, 0},
  };

  for (auto& [row, ans] : tests) {
    assert(Solution().minSwapsCouples(row) == ans);
  }
}