/*
 * @Date: 2023-12-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-19
 * @FilePath: /algorithm/cpp/1901_find_peak_grid/find_peak_grid.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findPeakGrid(vector<vector<int>>& mat) {
    int m = mat.size();
    int low = 0, high = m - 1;
    while (low <= high) {
      int i = (low + high) / 2;
      int j = max_element(mat[i].begin(), mat[i].end()) - mat[i].begin();
      if (i - 1 >= 0 && mat[i][j] < mat[i - 1][j]) {
        high = i - 1;
        continue;
      }
      if (i + 1 < m && mat[i][j] < mat[i + 1][j]) {
        low = i + 1;
        continue;
      }
      return {i, j};
    }
    return {};  // impossible
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{1, 4}, {3, 2}}, {0, 1}},
      {{{10, 20, 15}, {21, 30, 14}, {7, 16, 32}}, {1, 1}},
  };

  for (auto& [mat, ans] : tests) {
    assert(Solution().findPeakGrid(mat) == ans);
  }
}