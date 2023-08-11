/*
 * @Date: 2023-08-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-11
 * @FilePath: /algorithm/cpp/1572_diagonal_sum/diagonal_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int diagonalSum(vector<vector<int>>& mat) {
    int n = mat.size(), sum = 0, mid = n / 2;
    for (int i = 0; i < n; ++i) {
      sum += mat[i][i] + mat[i][n - 1 - i];
    }
    return sum - mat[mid][mid] * (n & 1);
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, 25},
      {{{1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}}, 8},
      {{{5}}, 5},
  };

  for (auto& [mat, ans] : tests) {
    assert(Solution().diagonalSum(mat) == ans);
  }
}