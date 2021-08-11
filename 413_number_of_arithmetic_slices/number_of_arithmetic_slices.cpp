/*
 * @Date: 2021-08-10 19:49:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-10 20:08:07
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int numberOfArithmeticSlices(vector<int>& nums) {
    int n = nums.size();
    if (n == 1) {
      return 0;
    }

    int d = nums[0] - nums[1], t = 0;
    int ans = 0;
    for (int i = 2; i < n; ++i) {
      if (nums[i - 1] - nums[i] == d) {
        ++t;
      } else {
        d = nums[i - 1] - nums[i];
        t = 0;
      }
      ans += t;
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{1, 2, 3, 4};
    assert(Solution{}.numberOfArithmeticSlices(nums) == 3);
  }

  {
    vector<int> nums{1};
    assert(Solution{}.numberOfArithmeticSlices(nums) == 0);
  }

  return 0;
}