/*
 * @Date: 2021-03-15 08:41:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-15 08:47:48
 * @FilePath: \algorithm\54_spiral_order\spiral_order.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

vector<int> spiral_order(vector<vector<int>>&& matrix) {
  int dx[4] = {0, 1, 0, -1};
  int dy[4] = {1, 0, -1, 0};

  int x = 0, y = 0, cur = 0;
  size_t size_x = matrix.size();
  size_t size_y = matrix[0].size();
  size_t all = size_x * size_y;
  vector<int> res;

  while (res.size() < all) {
    if (x >= 0 && x < size_x && y < size_y && matrix[x][y] != INT_MIN) {
      res.push_back(matrix[x][y]);
      matrix[x][y] = INT_MIN;
    } else {
      x -= dx[cur];
      y -= dy[cur];
      cur = (cur + 1) % 4;
    }

    x += dx[cur];
    y += dy[cur];
  }
  return res;
}

int main() {
  assert(spiral_order(vector<vector<int>>{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}) ==
         std::move(vector<int>{1, 2, 3, 6, 9, 8, 7, 4, 5}));
  assert(spiral_order(vector<vector<int>>{
             {1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}}) ==
         std::move(vector<int>{1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7}));
  return 0;
}