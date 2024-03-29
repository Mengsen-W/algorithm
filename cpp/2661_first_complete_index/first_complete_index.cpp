/*
 * @Date: 2023-12-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-01
 * @FilePath: /algorithm/cpp/2661_first_complete_index/first_complete_index.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int firstCompleteIndex(vector<int>& arr, vector<vector<int>>& mat) {
    int n = mat.size();
    int m = mat[0].size();
    unordered_map<int, pair<int, int>> mp;
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < m; ++j) {
        mp[mat[i][j]] = {i, j};
      }
    }
    vector<int> rowCnt(n, 0);
    vector<int> colCnt(m, 0);
    for (int i = 0; i < arr.size(); ++i) {
      auto& v = mp[arr[i]];
      ++rowCnt[v.first];
      if (rowCnt[v.first] == m) {
        return i;
      }
      ++colCnt[v.second];
      if (colCnt[v.second] == n) {
        return i;
      }
    }
    return -1;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, int>> tests{
      {{1, 3, 4, 2}, {{1, 4}, {2, 3}}, 2},
      {{2, 8, 7, 4, 1, 3, 5, 6, 9}, {{3, 2, 5}, {1, 4, 6}, {8, 7, 9}}, 3},
  };

  for (auto& [arr, mat, ans] : tests) {
    assert(Solution().firstCompleteIndex(arr, mat) == ans);
  }
}
