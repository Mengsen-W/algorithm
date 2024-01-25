/*
 * @Date: 2024-01-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-25
 * @FilePath: /algorithm/cpp/2859_sum_indices_with_k_set_bits/sum_indices_with_k_set_bits.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumIndicesWithKSetBits(vector<int>& nums, int k) {
    auto bitCount = [](int x) {
      x = (x & 0b0101010101) + ((x & 0b1010101010) >> 1);
      x = ((x & 0b0011001100) >> 2) + (x & 0b1100110011);
      x = (x >> 8) + ((x >> 4) & 0b1111) + (x & 0b1111);
      return x;
    };

    int ans = 0;
    for (int i = 0; i < nums.size(); ++i) {
      if (bitCount(i) == k) {
        ans += nums[i];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{5, 10, 1, 5, 2}, 1, 13},
      {{4, 3, 2, 1}, 2, 1},
  };

  for (auto& [nums, k, asn] : tests) {
    assert(Solution().sumIndicesWithKSetBits(nums, k) == asn);
  }
}
