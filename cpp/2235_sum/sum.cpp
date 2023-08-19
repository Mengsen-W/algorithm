/*
 * @Date: 2023-08-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-19
 * @FilePath: /algorithm/cpp/2235_sum/sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int sum(int num1, int num2) { return num1 + num2; }
};

int main() {
  std::vector<std::tuple<int, int, int>> tests{
      {12, 5, 17},
      {-10, 4, -6},
  };

  for (auto &[num1, num2, ans] : tests) {
    assert(Solution().sum(num1, num2) == ans);
  }
}