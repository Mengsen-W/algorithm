/*
 * @Date: 2022-10-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-27
 * @FilePath: /algorithm/1822_array_sign/array_sign.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int arraySign(vector<int>& nums) {
    int sign = 1;
    for (auto num : nums) {
      if (num == 0) {
        return 0;
      }
      if (num < 0) {
        sign = -sign;
      }
    }
    return sign;
  }
};

int main() {
  {
    vector<int> nums{-1, -2, -3, -4, 3, 2, 1};
    int ans = 1;
    assert(Solution().arraySign(nums) == ans);
  }

  {
    vector<int> nums{1, 5, 0, 2, -3};
    int ans = 0;
    assert(Solution().arraySign(nums) == ans);
  }

  {
    vector<int> nums{-1, 1, -1, 1, -1};
    int ans = -1;
    assert(Solution().arraySign(nums) == ans);
  }
}