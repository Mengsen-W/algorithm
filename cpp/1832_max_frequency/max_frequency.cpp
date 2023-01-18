/*
 * @Date: 2021-07-20 14:04:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-20 14:07:08
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxFrequency(vector<int>& nums, int k) {
    sort(nums.begin(), nums.end());
    int n = nums.size();
    long long total = 0;
    int l = 0, res = 1;
    for (int r = 1; r < n; ++r) {
      total += (long long)(nums[r] - nums[r - 1]) * (r - l);
      while (total > k) {
        total -= nums[r] - nums[l];
        ++l;
      }
      res = max(res, r - l + 1);
    }
    return res;
  }
};

int main() {
  {
    vector<int> nums{1, 2, 4};
    int k = 5;
    int ans = 3;
    assert(Solution{}.maxFrequency(nums, k) == ans);
  }
  {
    vector<int> nums{1, 4, 8, 13};
    int k = 5;
    int ans = 2;
    assert(Solution{}.maxFrequency(nums, k) == ans);
  }
}