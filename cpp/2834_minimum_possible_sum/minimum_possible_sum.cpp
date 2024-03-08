/*
 * @Date: 2024-03-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-08
 * @FilePath: /algorithm/cpp/2834_minimum_possible_sum/minimum_possible_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int minimumPossibleSum(int n, int target) {
    const int mod = 1e9 + 7;
    int m = target / 2;
    if (n <= m) {
      return (long long)(1 + n) * n / 2 % mod;
    }
    return ((long long)(1 + m) * m / 2 + ((long long)target + target + (n - m) - 1) * (n - m) / 2) % mod;
  }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {2, 3, 4},
      {3, 3, 8},
      {1, 1, 1},
  };

  for (auto &[n, target, ans] : tests) {
    assert(Solution().minimumPossibleSum(n, target) == ans);
  }
}