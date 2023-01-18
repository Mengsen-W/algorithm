/*
 * @Date: 2022-08-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-04
 * @FilePath: /algorithm/1403_min_subsequence/min_subsequence.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> minSubsequence(vector<int>& nums) {
    int total = accumulate(nums.begin(), nums.end(), 0);
    sort(nums.begin(), nums.end());
    vector<int> ans;
    int curr = 0;
    for (int i = nums.size() - 1; i >= 0; --i) {
      curr += nums[i];
      ans.emplace_back(nums[i]);
      if (total - curr < curr) {
        break;
      }
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{4, 3, 10, 9, 8};
    vector<int> ans{10, 9};
    assert(Solution().minSubsequence(nums) == ans);
  }

  {
    vector<int> nums{4, 4, 7, 6, 7};
    vector<int> ans{7, 7, 6};
    assert(Solution().minSubsequence(nums) == ans);
  }
}