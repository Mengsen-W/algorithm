/*
 * @Date: 2023-10-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-17
 * @FilePath: /algorithm/cpp/2652_sum_of_multiples/sum_of_multiples.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <vector>

class Solution {
 public:
  int sumOfMultiples(int n) {
    std::function<int(int, int)> f = [](int n, int m) -> int { return (m + n / m * m) * (n / m) / 2; };
    return f(n, 3) + f(n, 5) + f(n, 7) - f(n, 3 * 5) - f(n, 3 * 7) - f(n, 5 * 7) + f(n, 3 * 5 * 7);
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {7, 21},
      {10, 40},
      {9, 30},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().sumOfMultiples(n) == ans);
  }
}