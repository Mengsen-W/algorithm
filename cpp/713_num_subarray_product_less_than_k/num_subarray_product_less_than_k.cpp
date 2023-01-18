/*
 * @Date: 2022-05-05 09:40:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-05 09:44:30
 * @FilePath: /algorithm/713_num_subarray_product_less_than_k/num_subarray_product_less_than_k.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int numSubarrayProductLessThanK(vector<int> nums, int k) {
    int n = nums.size(), ret = 0;
    int prod = 1, i = 0;
    for (int j = 0; j < n; j++) {
      prod *= nums[j];
      while (i <= j && prod >= k) {
        prod /= nums[i];
        i++;
      }
      ret += j - i + 1;
    }
    return ret;
  }
};

int main() {
  assert(Solution().numSubarrayProductLessThanK(vector<int>{10, 5, 2, 6}, 100) == 8);
  assert(Solution().numSubarrayProductLessThanK(vector<int>{1, 2, 3}, 0) == 0);
}