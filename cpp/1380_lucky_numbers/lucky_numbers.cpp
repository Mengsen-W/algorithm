/*
 * @Date: 2022-02-15 02:05:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-15 02:20:41
 * @FilePath: /algorithm/1380_lucky_numbers/lucky_numbers.cpp
 */

#include <cassert>
#include <climits>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> luckyNumbers(vector<vector<int>> matrix) {
    int m = matrix.size(), n = matrix[0].size();
    vector<int> minRow(m, INT_MAX), maxCol(n);
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        minRow[i] = min(minRow[i], matrix[i][j]);
        maxCol[j] = max(maxCol[j], matrix[i][j]);
      }
    }
    vector<int> ret;
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        if (matrix[i][j] == minRow[i] && matrix[i][j] == maxCol[j]) {
          ret.push_back(matrix[i][j]);
        }
      }
    }
    return ret;
  }
};

int main() {
  assert(Solution().luckyNumbers({{3, 7, 8}, {9, 11, 13}, {15, 16, 17}}) ==
         vector<int>({15}));
  assert(Solution().luckyNumbers(
             {{1, 10, 4, 2}, {9, 3, 8, 7}, {15, 16, 17, 12}}) ==
         vector<int>({12}));
  assert(Solution().luckyNumbers({{7, 8}, {1, 2}}) == vector<int>({7}));
}