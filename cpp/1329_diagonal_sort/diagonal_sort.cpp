/*
 * @Date: 2024-04-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-29
 * @FilePath: /algorithm/cpp/1329_diagonal_sort/diagonal_sort.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> diagonalSort(vector<vector<int>>& mat) {
    int n = mat.size(), m = mat[0].size();
    vector<vector<int>> diag(m + n);
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < m; ++j) {
        diag[i - j + m].push_back(mat[i][j]);
      }
    }
    for (auto& d : diag) {
      sort(d.rbegin(), d.rend());
    }
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < m; ++j) {
        mat[i][j] = diag[i - j + m].back();
        diag[i - j + m].pop_back();
      }
    }
    return mat;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>>> tests{
      {{{3, 3, 1, 1}, {2, 2, 1, 2}, {1, 1, 1, 2}}, {{1, 1, 1, 1}, {1, 2, 2, 2}, {1, 2, 3, 3}}},
      {{{11, 25, 66, 1, 69, 7},
        {23, 55, 17, 45, 15, 52},
        {75, 31, 36, 44, 58, 8},
        {22, 27, 33, 25, 68, 4},
        {84, 28, 14, 11, 5, 50}},
       {{5, 17, 4, 1, 52, 7},
        {11, 11, 25, 45, 8, 69},
        {14, 23, 25, 44, 58, 15},
        {22, 27, 31, 36, 50, 66},
        {84, 28, 75, 33, 55, 68}}},
  };

  for (auto& [mat, ans] : tests) {
    assert(Solution().diagonalSort(mat) == ans);
  }
}