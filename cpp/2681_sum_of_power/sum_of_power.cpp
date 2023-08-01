/*
 * @Date: 2023-08-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-01
 * @FilePath: /algorithm/cpp/2681_sum_of_power/sum_of_power.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumOfPower(vector<int>& nums) {
    int n = nums.size();
    sort(nums.begin(), nums.end());
    int dp = 0, preSum = 0;
    int res = 0, mod = 1e9 + 7;
    for (int i = 0; i < n; i++) {
      dp = (nums[i] + preSum) % mod;
      preSum = (preSum + dp) % mod;
      res = (int)((res + (long long)nums[i] * nums[i] % mod * dp) % mod);
      if (res < 0) {
        res += mod;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{2, 1, 4}, 141},
      {{1, 1, 1}, 7},
  };

  for (auto& [nums, target] : tests) {
    assert(Solution().sumOfPower(nums) == target);
  }
}