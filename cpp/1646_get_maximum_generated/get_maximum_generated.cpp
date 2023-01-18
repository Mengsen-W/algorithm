/*
 * @Date: 2021-08-23 21:10:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-23 21:13:13
 */

#include <cassert>
#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  int getMaximumGenerated(int n) {
    if (n == 0) {
      return 0;
    }
    vector<int> nums(n + 1);
    nums[1] = 1;
    for (int i = 2; i <= n; ++i) {
      nums[i] = nums[i / 2] + i % 2 * nums[i / 2 + 1];
    }
    return *max_element(nums.begin(), nums.end());
  }
};

int main() {
  assert(Solution().getMaximumGenerated(7) == 3);
  assert(Solution().getMaximumGenerated(2) == 1);
  assert(Solution().getMaximumGenerated(3) == 2);
  return 0;
}