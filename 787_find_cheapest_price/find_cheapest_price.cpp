/*
 * @Date: 2021-08-24 09:45:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-24 10:33:05
 */

#include <cassert>
#include <vector>
using namespace std;

class Solution {
 public:
  int findCheapestPrice(int n, vector<vector<int>>& flights, int src, int dst,
                        int k) {
    // 中转k次就是走过两次
    static constexpr int INF = 10000 * 101 + 1;
    // f[t][i] 经过 t 次航班，从源站到i的最小花费
    vector<vector<int>> f(k + 2, vector<int>(n, INF));
    // 经过0航班直接到达源站
    f[0][src] = 0;

    for (int t = 1; t <= k + 1; ++t) {
      for (auto&& flight : flights) {
        int j = flight[0], i = flight[1], cost = flight[2];
        f[t][i] = min(f[t][i], f[t - 1][j] + cost);
      }
    }
    int ans = INF;
    for (int t = 1; t <= k + 1; ++t) {
      ans = min(ans, f[t][dst]);
    }
    return (ans == INF ? -1 : ans);
  }
};

int main() {
  {
    int n = 3;
    vector<vector<int>> edges{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}};
    int src = 0;
    int dst = 2;
    int k = 1;
    assert(Solution{}.findCheapestPrice(n, edges, src, dst, k) == 200);
  }
  {
    int n = 3;
    vector<vector<int>> edges{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}};
    int src = 0;
    int dst = 2;
    int k = 0;
    assert(Solution{}.findCheapestPrice(n, edges, src, dst, k) == 500);
  }
}