/*
 * @Date: 2022-11-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-18
 * @FilePath: /algorithm/891_sum_subseq_widths/sum_subseq_widths.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumSubseqWidths(vector<int>& nums) {
    sort(nums.begin(), nums.end());
    long long res = 0, mod = 1e9 + 7;
    long long x = nums[0], y = 2;
    for (int j = 1; j < nums.size(); j++) {
      res = (res + nums[j] * (y - 1) - x) % mod;
      x = (x * 2 + nums[j]) % mod;
      y = y * 2 % mod;
    }
    return (res + mod) % mod;
  }
};

int main() {
  {
    vector<int> nums{2, 1, 3};
    int ans = 6;
    assert(Solution().sumSubseqWidths(nums) == ans);
  }

  {
    vector<int> nums{2};
    int ans = 0;
    assert(Solution().sumSubseqWidths(nums) == ans);
  }
}