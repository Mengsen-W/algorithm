/*
 * @Date: 2023-10-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-26
 * @FilePath: /algorithm/cpp/2520_count_digits/count_digits.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int countDigits(int num) {
    int t = num, res = 0;
    while (t) {
      if (num % (t % 10) == 0) {
        res += 1;
      }
      t /= 10;
    }
    return res;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {7, 1},
      {121, 2},
      {1248, 4},
  };

  for (auto& [num, ans] : tests) {
    assert(Solution().countDigits(num) == ans);
  }
}