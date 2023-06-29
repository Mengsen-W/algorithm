/*
 * @Date: 2023-06-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-29
 * @FilePath: /algorithm/cpp/1253_reconstruct_matrix/reconstruct_matrix.cpp
 */

#include <cassert>
#include <iostream>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> reconstructMatrix(int upper, int lower, vector<int>& colsum) {
    int n = colsum.size();
    int sum = 0, two = 0;
    for (int i = 0; i < n; ++i) {
      if (colsum[i] == 2) {
        ++two;
      }
      sum += colsum[i];
    }
    if (sum != upper + lower || min(upper, lower) < two) {
      return {};
    }
    upper -= two;
    lower -= two;
    vector<vector<int>> res(2, vector<int>(n, 0));
    for (int i = 0; i < n; ++i) {
      if (colsum[i] == 2) {
        res[0][i] = res[1][i] = 1;
      } else if (colsum[i] == 1) {
        if (upper > 0) {
          res[0][i] = 1;
          --upper;
        } else {
          res[1][i] = 1;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<int, int, vector<int>, vector<vector<int>>>> testMap{
      {2, 1, vector<int>{1, 1, 1}, vector<vector<int>>{{1, 1, 0}, {0, 0, 1}}},
      {2, 3, vector<int>{2, 2, 1, 1}, vector<vector<int>>{}},
      {5, 5, vector<int>{2, 1, 2, 0, 1, 0, 1, 2, 0, 1},
       vector<vector<int>>{{1, 1, 1, 0, 1, 0, 0, 1, 0, 0}, {1, 0, 1, 0, 0, 0, 1, 1, 0, 1}}},
  };

  for (auto& [upper, lower, colsum, ans] : testMap) {
    assert(Solution().reconstructMatrix(upper, lower, colsum) == ans);
  }
}