/*
 * @Date: 2023-07-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-05
 * @FilePath: /algorithm/cpp/2600_k_items_with_maximum_sum/k_items_with_maximum_sum.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

class Solution {
 public:
  int kItemsWithMaximumSum(int numOnes, int numZeros, int numNegOnes, int k) {
    if (k <= numOnes) {
      return k;
    } else if (k <= numOnes + numZeros) {
      return numOnes;
    } else {
      return numOnes - (k - numOnes - numZeros);
    }
  }
};

int main() {
  std::vector<std::tuple<int, int, int, int, int>> testMap{
      {3, 2, 0, 2, 2},
      {3, 2, 0, 4, 3},
  };

  for (auto &[numOnes, numZeros, numNegOnes, k, expected] : testMap) {
    assert(Solution().kItemsWithMaximumSum(numNegOnes, numZeros, numNegOnes, k) == expected);
  }
}