/*
 * @Date: 2022-11-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-16
 * @FilePath: /algorithm/775_is_ideal_permutation/is_ideal_permutation.cpp
 */

#include <cassert>
#include <cmath>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isIdealPermutation(vector<int>& nums) {
    for (int i = 0; i < nums.size(); i++) {
      if (abs(nums[i] - i) > 1) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  {
    vector<int> nums{1, 0, 2};
    assert(Solution().isIdealPermutation(nums));
  }

  {
    vector<int> nums{1, 2, 0};
    assert(!Solution().isIdealPermutation(nums));
  }
}
