/*
 * @Date: 2023-08-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-31
 * @FilePath: /algorithm/cpp/1761_min_trio_degree/min_trio_degree.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int minTrioDegree(int n, vector<vector<int>>& edges) {
    vector<vector<int>> g(n, vector<int>(n));
    vector<int> degree(n);

    for (auto&& edge : edges) {
      int x = edge[0] - 1, y = edge[1] - 1;
      g[x][y] = g[y][x] = 1;
      ++degree[x];
      ++degree[y];
    }

    int ans = INT_MAX;
    for (int i = 0; i < n; ++i) {
      for (int j = i + 1; j < n; ++j) {
        if (g[i][j] == 1) {
          for (int k = j + 1; k < n; ++k) {
            if (g[i][k] == 1 && g[j][k] == 1) {
              ans = min(ans, degree[i] + degree[j] + degree[k] - 6);
            }
          }
        }
      }
    }

    return ans == INT_MAX ? -1 : ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {6, {{1, 2}, {1, 3}, {3, 2}, {4, 1}, {5, 2}, {3, 6}}, 3},
      {7, {{1, 3}, {4, 1}, {4, 3}, {2, 5}, {5, 6}, {6, 7}, {7, 5}, {2, 6}}, 0},
  };

  for (auto& [n, edges, ans] : tests) {
    assert(Solution().minTrioDegree(n, edges) == ans);
  }
}