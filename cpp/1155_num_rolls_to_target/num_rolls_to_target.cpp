/*
 * @Date: 2023-10-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-24
 * @FilePath: /algorithm/cpp/1155_num_rolls_to_target/num_rolls_to_target.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int numRollsToTarget(int n, int k, int target) {
    vector<int> f(target + 1);
    f[0] = 1;
    for (int i = 1; i <= n; ++i) {
      for (int j = target; j >= 0; --j) {
        f[j] = 0;
        for (int x = 1; x <= k; ++x) {
          if (j - x >= 0) {
            f[j] = (f[j] + f[j - x]) % mod;
          }
        }
      }
    }
    return f[target];
  }

 private:
  static constexpr int mod = 1000000007;
};

int main() {
  vector<tuple<int, int, int, int>> tests{
      {1, 6, 3, 1},
      {2, 6, 7, 6},
      {30, 30, 500, 222616187},
  };

  for (auto &[n, k, target, ans] : tests) {
    assert(Solution().numRollsToTarget(n, k, target) == ans);
  }
}