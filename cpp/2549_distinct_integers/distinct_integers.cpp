/*
 * @Date: 2024-03-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-23
 * @FilePath: /algorithm/cpp/2549_distinct_integers/distinct_integers.cpp
 */

#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int distinctIntegers(int n) { return std::max(1, n - 1); }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {5, 4},
      {3, 2},
  };

  for (auto &[n, expected] : tests) {
    assert(Solution().distinctIntegers(n) == expected);
  }
}