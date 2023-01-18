/*
 * @Date: 2021-11-11 01:04:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-11 01:08:34
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 private:
  static constexpr int mod = 1000000007;

 public:
  int kInversePairs(int n, int k) {
    vector<vector<int>> f(2, vector<int>(k + 1));
    f[0][0] = 1;
    for (int i = 1; i <= n; ++i)
      for (int j = 0; j <= k; ++j) {
        int cur = i & 1, prev = cur ^ 1;
        f[cur][j] = (j - 1 >= 0 ? f[cur][j - 1] : 0) -
                    (j - i >= 0 ? f[prev][j - i] : 0) + f[prev][j];
        if (f[cur][j] >= mod)
          f[cur][j] -= mod;
        else if (f[cur][j] < 0)
          f[cur][j] += mod;
      }
    return f[n & 1][k];
  }
};

int main() {
  assert(Solution().kInversePairs(3, 0) == 1);
  assert(Solution().kInversePairs(3, 1) == 2);
}