/*
 * @Date: 2023-06-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-22
 * @FilePath: /algorithm/cpp/16.19_pond_sizes/pond_sizes.cpp
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> pondSizes(vector<vector<int>>& land) {
    int m = land.size(), n = land[0].size();
    auto bfs = [&](int x, int y) -> int {
      int res = 0;
      queue<pair<int, int>> q;
      q.push({x, y});
      land[x][y] = -1;
      while (!q.empty()) {
        auto [x, y] = q.front();
        q.pop();
        res++;
        for (int dx = -1; dx <= 1; dx++) {
          for (int dy = -1; dy <= 1; dy++) {
            if (dx == 0 && dy == 0) {
              continue;
            }
            if (x + dx < 0 || x + dx >= m || y + dy < 0 || y + dy >= n || land[x + dx][y + dy] != 0) {
              continue;
            }
            land[x + dx][y + dy] = -1;
            q.push({x + dx, y + dy});
          }
        }
      }
      return res;
    };

    vector<int> res;
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (land[i][j] == 0) {
          res.push_back(bfs(i, j));
        }
      }
    }
    sort(res.begin(), res.end());
    return res;
  }
};

int main() {
  vector<vector<int>> land{{0, 2, 1, 0}, {0, 1, 0, 1}, {1, 1, 0, 1}, {0, 1, 0, 1}};
  vector<int> ans{1,2,4};
  assert(Solution().pondSizes(land) == ans);
}