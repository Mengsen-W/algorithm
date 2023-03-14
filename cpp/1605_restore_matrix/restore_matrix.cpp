/*
 * @Date: 2023-03-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-14
 * @FilePath: /algorithm/cpp/1605_restore_matrix/restore_matrix.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> restoreMatrix(vector<int>& rowSum, vector<int>& colSum) {
    int n = rowSum.size(), m = colSum.size();
    vector<vector<int>> matrix(n, vector<int>(m, 0));
    int i = 0, j = 0;
    while (i < n && j < m) {
      int v = min(rowSum[i], colSum[j]);
      matrix[i][j] = v;
      rowSum[i] -= v;
      colSum[j] -= v;
      if (rowSum[i] == 0) {
        ++i;
      }
      if (colSum[j] == 0) {
        ++j;
      }
    }
    return matrix;
  }
};

int main() {
  {
    vector<int> rowSum{3, 8};
    vector<int> colSum{4, 7};
    vector<vector<int>> ans{{3, 0}, {1, 7}};
    assert(Solution().restoreMatrix(rowSum, colSum) == ans);
  }

  {
    vector<int> rowSum{5, 7, 10};
    vector<int> colSum{8, 6, 8};
    vector<vector<int>> ans{{5, 0, 0}, {3, 4, 0}, {0, 2, 8}};
    assert(Solution().restoreMatrix(rowSum, colSum) == ans);
  }

  {
    vector<int> rowSum{14, 9};
    vector<int> colSum{6, 9, 8};
    vector<vector<int>> ans{{6, 8, 0}, {0, 1, 8}};
    assert(Solution().restoreMatrix(rowSum, colSum) == ans);
  }

  {
    vector<int> rowSum{1, 0};
    vector<int> colSum{1};
    vector<vector<int>> ans{{1}, {0}};
    assert(Solution().restoreMatrix(rowSum, colSum) == ans);
  }

  {
    vector<int> rowSum{0};
    vector<int> colSum{0};
    vector<vector<int>> ans{{0}};
    assert(Solution().restoreMatrix(rowSum, colSum) == ans);
  }
}