/*
 * @Date: 2022-06-14 09:51:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-14 09:57:49
 * @FilePath: /algorithm/498_find_diagonal_order/find_diagonal_order.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findDiagonalOrder(vector<vector<int>> mat) {
    int m = mat.size();
    int n = mat[0].size();
    vector<int> res;
    for (int i = 0; i < m + n - 1; i++) {
      if (i % 2) {
        int x = i < n ? 0 : i - n + 1;
        int y = i < n ? i : n - 1;
        while (x < m && y >= 0) {
          res.emplace_back(mat[x][y]);
          x++;
          y--;
        }
      } else {
        int x = i < m ? i : m - 1;
        int y = i < m ? 0 : i - m + 1;
        while (x >= 0 && y < n) {
          res.emplace_back(mat[x][y]);
          x--;
          y++;
        }
      }
    }
    return res;
  }
};

int main() {
  assert((Solution().findDiagonalOrder({{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}) == vector<int>{1, 2, 4, 7, 5, 3, 6, 8, 9}));
  assert((Solution().findDiagonalOrder({{1, 2}, {3, 4}}) == vector<int>{1, 2, 3, 4}));
}
