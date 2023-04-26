/*
 * @Date: 2023-04-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-26
 * @FilePath: /algorithm/cpp/1031_max_sum_two_no_overlap/max_sum_two_no_overlap.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  int help(vector<int>& nums, int firstLen, int secondLen) {
    int suml = accumulate(nums.begin(), nums.begin() + firstLen, 0);
    int maxSumL = suml;
    int sumr = accumulate(nums.begin() + firstLen, nums.begin() + firstLen + secondLen, 0);
    int res = maxSumL + sumr;
    for (int i = firstLen + secondLen, j = firstLen; i < nums.size(); ++i, ++j) {
      suml += nums[j] - nums[j - firstLen];
      maxSumL = max(maxSumL, suml);
      sumr += nums[i] - nums[i - secondLen];
      res = max(res, maxSumL + sumr);
    }
    return res;
  }

  int maxSumTwoNoOverlap(vector<int>& nums, int firstLen, int secondLen) {
    return max(help(nums, firstLen, secondLen), help(nums, secondLen, firstLen));
  }
};

int main() {
  {
    vector<int> nums{0, 6, 5, 2, 2, 5, 1, 9, 4};
    int firstLen = 1;
    int secondLen = 2;
    int ans = 20;
    assert(Solution().maxSumTwoNoOverlap(nums, firstLen, secondLen) == ans);
  }

  {
    vector<int> nums{3,8,1,3,2,1,8,9,0};
    int firstLen = 3;
    int secondLen = 2;
    int ans = 29;
    assert(Solution().maxSumTwoNoOverlap(nums, firstLen, secondLen) == ans);
  }

  {
    vector<int> nums{2,1,5,6,0,9,5,0,3,8};
    int firstLen = 4;
    int secondLen = 3;
    int ans = 31;
    assert(Solution().maxSumTwoNoOverlap(nums, firstLen, secondLen) == ans);
  }
}