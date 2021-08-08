/*
 * @Date: 2021-08-08 11:40:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-08 13:21:22
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int tribonacci(int n) {
    if (n == 0) return 0;

    if (n <= 2) return 1;

    vector<vector<long>> q = {{1, 1, 1}, {1, 0, 0}, {0, 1, 0}};
    vector<vector<long>> res = pow(q, n);
    return res[0][2];
  }

  vector<vector<long>> pow(vector<vector<long>>& a, long n) {
    vector<vector<long>> ret = {{1, 0, 0}, {0, 1, 0}, {0, 0, 1}};
    while (n > 0) {
      if ((n & 1) == 1) ret = multiply(ret, a);
      n >>= 1;
      a = multiply(a, a);
    }
    return ret;
  }

  vector<vector<long>> multiply(vector<vector<long>>& a,
                                vector<vector<long>>& b) {
    vector<vector<long>> c(3, vector<long>(3));
    for (int i = 0; i < 3; i++)
      for (int j = 0; j < 3; j++)
        c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];

    return c;
  }
};

int main() {
  assert(Solution{}.tribonacci(4) == 4);
  assert(Solution{}.tribonacci(25) == 1389537);

  return 0;
}