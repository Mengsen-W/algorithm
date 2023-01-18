/*
 * @Date: 2022-08-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-29
 * @FilePath: /algorithm/1470_shuffle/shuffle.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> shuffle(vector<int>& nums, int n) {
    vector<int> ans(2 * n);
    for (int i = 0; i < n; i++) {
      ans[2 * i] = nums[i];
      ans[2 * i + 1] = nums[i + n];
    }
    return ans;
  }
};

int main() {
  {
    vector<int> nums{2, 5, 1, 3, 4, 7};
    int n = 3;
    vector<int> ans{2, 3, 5, 4, 1, 7};
    assert(Solution().shuffle(nums, n) == ans);
  }

  {
    vector<int> nums{1, 2, 3, 4, 4, 3, 2, 1};
    int n = 4;
    vector<int> ans{1, 4, 2, 3, 3, 2, 4, 1};
    assert(Solution().shuffle(nums, n) == ans);
  }

  {
    vector<int> nums{1, 1, 2, 2};
    int n = 2;
    vector<int> ans{1, 2, 1, 2};
    assert(Solution().shuffle(nums, n) == ans);
  }
}