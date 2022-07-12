/*
 * @Date: 2022-07-12
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-12
 * @FilePath: /algorithm/1252_odd_cells/odd_cells.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int oddCells(int m, int n, vector<vector<int>> indices) {
    vector<int> rows(m), cols(n);
    for (auto& index : indices) {
      rows[index[0]]++;
      cols[index[1]]++;
    }
    int oddx = 0, oddy = 0;
    for (int i = 0; i < m; i++) {
      if (rows[i] & 1) {
        oddx++;
      }
    }
    for (int i = 0; i < n; i++) {
      if (cols[i] & 1) {
        oddy++;
      }
    }
    return oddx * (n - oddy) + (m - oddx) * oddy;
  }
};

int main() {
  assert(Solution().oddCells(2, 3, vector<vector<int>>{{0, 1}, {1, 1}}) == 6);
  assert(Solution().oddCells(2, 2, vector<vector<int>>{{1, 1}, {0, 0}}) == 0);
}