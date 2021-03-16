/*
 * @Date: 2021-03-16 09:33:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-16 17:19:46
 */

#include <cassert>
#include <vector>

using namespace std;

vector<vector<int>> generate_matrix(int n) {
  // directory (x, y)
  int dx[4] = {0, 1, 0, -1};
  int dy[4] = {1, 0, -1, 0};
  // round % 4
  int x = 0, y = 0, d = 0;
  // '0' be used to test boundary
  vector<vector<int>> ans(n, vector<int>(n, 0));

  for (int i = 1; i <= n * n; ++i) {
    ans[x][y] = i;

    int tx = x + dx[d];
    int ty = y + dy[d];

    if (tx < 0 || tx >= n || ty < 0 || ty >= n || ans[tx][ty] != 0) {
      d = (d + 1) % 4;
      tx = x + dx[d];
      ty = y + dy[d];
    }

    x = tx;
    y = ty;
  }

  return ans;
}

int main(int argc, char* argv[]) {
  assert(generate_matrix(3) ==
         std::move(vector<vector<int>>{{1, 2, 3}, {8, 9, 4}, {7, 6, 5}}));
  assert(generate_matrix(1) == std::move(vector<vector<int>>{{1}}));
  return 0;
}
