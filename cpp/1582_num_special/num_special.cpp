/*
 * @Date: 2022-09-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-04
 * @FilePath: /algorithm/1582_num_special/num_special.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int numSpecial(vector<vector<int>>& mat) {
    int m = mat.size(), n = mat[0].size();
    for (int i = 0; i < m; i++) {
      int cnt1 = 0;
      for (int j = 0; j < n; j++) {
        if (mat[i][j] == 1) {
          cnt1++;
        }
      }
      if (i == 0) {
        cnt1--;
      }
      if (cnt1 > 0) {
        for (int j = 0; j < n; j++) {
          if (mat[i][j] == 1) {
            mat[0][j] += cnt1;
          }
        }
      }
    }
    int sum = 0;
    for (int i = 0; i < n; i++) {
      if (mat[0][i] == 1) {
        sum++;
      }
    }
    return sum;
  }
};

int main() {
  {
    vector<vector<int>> mat{{1, 0, 0}, {0, 0, 1}, {1, 0, 0}};
    int ans = 1;
    assert(Solution().numSpecial(mat) == ans);
  }

  {
    vector<vector<int>> mat{{1, 0, 0}, {0, 1, 0}, {0, 0, 1}};
    int ans = 3;
    assert(Solution().numSpecial(mat) == ans);
  }

  {
    vector<vector<int>> mat{{0, 0, 0, 1}, {1, 0, 0, 0}, {0, 1, 1, 0}, {0, 0, 0, 0}};
    int ans = 2;
    assert(Solution().numSpecial(mat) == ans);
  }

  {
    vector<vector<int>> mat{{0, 0, 0, 0, 0}, {1, 0, 0, 0, 0}, {0, 1, 0, 0, 0}, {0, 0, 1, 0, 0}, {0, 0, 0, 1, 1}};

    int ans = 3;
    assert(Solution().numSpecial(mat) == ans);
  }
}