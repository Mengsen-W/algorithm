/*
 * @Date: 2021-07-20 14:13:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-20 14:17:20
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int minPairSum(vector<int>& nums) {
    int n = nums.size();
    int res = 0;
    sort(nums.begin(), nums.end());
    for (int i = 0; i < n / 2; ++i) {
      res = max(res, nums[i] + nums[n - 1 - i]);
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{3, 5, 2, 3};
    int ans = 7;
    assert(Solution{}.minPairSum(nums) == ans);
  }
  {
    vector<int> nums{3, 5, 4, 2, 4, 6};
    int ans = 8;
    assert(Solution{}.minPairSum(nums) == ans);
  }
}