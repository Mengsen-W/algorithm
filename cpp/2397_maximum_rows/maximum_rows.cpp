/*
 * @Date: 2024-01-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-04
 * @FilePath: /algorithm/cpp/2397_maximum_rows/maximum_rows.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumRows(vector<vector<int>>& matrix, int numSelect) {
    int m = matrix.size();
    int n = matrix[0].size();
    vector<int> mask(m, 0);
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        mask[i] += matrix[i][j] << (n - j - 1);
      }
    }
    int res = 0;
    int cur = (1 << numSelect) - 1;
    int limit = (1 << n);
    while (cur < limit) {
      int t = 0;
      for (int j = 0; j < m; j++) {
        if ((mask[j] & cur) == mask[j]) {
          ++t;
        }
      }
      res = max(res, t);
      int lb = cur & -cur;
      int r = cur + lb;
      cur = ((r ^ cur) >> (__builtin_ctz(lb) + 2)) | r;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{0, 0, 0}, {1, 0, 1}, {0, 1, 1}, {0, 0, 1}}, 2, 3},
      {{{1}, {0}}, 1, 2},
  };

  for (auto& [matrix, numSelect, ans] : tests) {
    assert(Solution().maximumRows(matrix, numSelect) == ans);
  }
}