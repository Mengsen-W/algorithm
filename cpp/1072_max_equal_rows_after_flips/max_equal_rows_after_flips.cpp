/*
 * @Date: 2023-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-16
 * @FilePath: /algorithm/cpp/1072_max_equal_rows_after_flips/max_equal_rows_after_flips.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxEqualRowsAfterFlips(vector<vector<int>>& matrix) {
    int m = matrix.size(), n = matrix[0].size();
    unordered_map<string, int> mp;
    for (int i = 0; i < m; i++) {
      string s = string(n, '0');
      for (int j = 0; j < n; j++) {
        // 如果 matrix[i][0] 为 1，则对该行元素进行翻转
        s[j] = '0' + (matrix[i][j] ^ matrix[i][0]);
      }
      mp[s]++;
    }
    int res = 0;
    for (auto& [k, v] : mp) {
      res = max(res, v);
    }
    return res;
  }
};

int main() {
  {
    vector<vector<int>> matrix{{0, 1}, {1, 1}};
    int ans = 1;
    assert(Solution().maxEqualRowsAfterFlips(matrix) == ans);
  }

  {
    vector<vector<int>> matrix{{0, 1}, {1, 0}};
    int ans = 2;
    assert(Solution().maxEqualRowsAfterFlips(matrix) == ans);
  }

  {
    vector<vector<int>> matrix{{0, 0, 0}, {0, 0, 1}, {1, 1, 0}};
    int ans = 2;
    assert(Solution().maxEqualRowsAfterFlips(matrix) == ans);
  }
}