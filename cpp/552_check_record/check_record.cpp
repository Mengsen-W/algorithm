/*
 * @Date: 2021-08-18 08:36:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-18 21:29:10
 */

#include <cassert>
#include <numeric>
#include <vector>
using namespace std;

class Solution {
 public:
  static constexpr int MOD = 1'000'000'007;

  vector<vector<long>> pow(vector<vector<long>> mat, int n) {
    vector<vector<long>> ret = {{1, 0, 0, 0, 0, 0}};
    while (n > 0) {
      if ((n & 1) == 1) {
        ret = multiply(ret, mat);
      }
      n >>= 1;
      mat = multiply(mat, mat);
    }
    return ret;
  }

  vector<vector<long>> multiply(vector<vector<long>> a, vector<vector<long>> b) {
    int rows = a.size(), columns = b[0].size(), temp = b.size();
    vector<vector<long>> c(rows, vector<long>(columns));
    for (int i = 0; i < rows; i++) {
      for (int j = 0; j < columns; j++) {
        for (int k = 0; k < temp; k++) {
          c[i][j] += a[i][k] * b[k][j];
          c[i][j] %= MOD;
        }
      }
    }
    return c;
  }

  int checkRecord(int n) {
    vector<vector<long>> mat = {{1, 1, 0, 1, 0, 0}, {1, 0, 1, 1, 0, 0}, {1, 0, 0, 1, 0, 0},
                                {0, 0, 0, 1, 1, 0}, {0, 0, 0, 1, 0, 1}, {0, 0, 0, 1, 0, 0}};
    vector<vector<long>> res = pow(mat, n);
    long sum = accumulate(res[0].begin(), res[0].end(), 0ll);
    return (int)(sum % MOD);
  }
};

int main() {
  assert(Solution{}.checkRecord(2) == 8);
  assert(Solution{}.checkRecord(1) == 3);
  assert(Solution{}.checkRecord(10101) == 183236316);
}