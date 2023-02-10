/*
 * @Date: 2023-02-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-10
 * @FilePath: /algorithm/cpp/1223_die_simulator/die_simulator.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  static constexpr int mod = 1E9 + 7;
  int dieSimulator(int n, vector<int>& rollMax) {
    vector<vector<int>> d(n + 1, vector<int>(6, 0));
    vector<int> sum(n + 1, 0);
    sum[0] = 1;
    for (int i = 1; i <= n; i++) {
      for (int j = 0; j < 6; j++) {
        int pos = max(i - rollMax[j] - 1, 0);
        int sub = ((sum[pos] - d[pos][j]) % mod + mod) % mod;
        d[i][j] = ((sum[i - 1] - sub) % mod + mod) % mod;
        if (i <= rollMax[j]) {
          d[i][j] = (d[i][j] + 1) % mod;
        }
        sum[i] = (sum[i] + d[i][j]) % mod;
      }
    }
    return sum[n];
  }
};

int main() {
  {
    int n = 2;
    vector<int> rollMax{1, 1, 2, 2, 2, 3};
    int ans = 34;
    assert(Solution().dieSimulator(n, rollMax) == ans);
  }

  {
    int n = 2;
    vector<int> rollMax{1, 1, 1, 1, 1, 1};
    int ans = 30;
    assert(Solution().dieSimulator(n, rollMax) == ans);
  }

  {
    int n = 3;
    vector<int> rollMax{1, 1, 1, 2, 2, 3};
    int ans = 181;
    assert(Solution().dieSimulator(n, rollMax) == ans);
  }
}