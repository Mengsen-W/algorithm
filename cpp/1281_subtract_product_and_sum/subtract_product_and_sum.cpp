/*
 * @Date: 2023-08-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-09
 * @FilePath: /algorithm/cpp/1281_subtract_product_and_sum/subtract_product_and_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int subtractProductAndSum(int n) {
    int m = 1, s = 0;
    while (n) {
      int x = n % 10;
      n /= 10;
      m *= x;
      s += x;
    }
    return m - s;
  }
};

int main() {
  std::vector<std::tuple<int, int>> tests{
      {234, 15},
      {4421, 21},
  };

  for (auto &[n, ans] : tests) {
    assert(Solution().subtractProductAndSum(n) == ans);
  }
}