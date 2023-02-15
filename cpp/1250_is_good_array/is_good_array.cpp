/*
 * @Date: 2023-02-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-15
 * @FilePath: /algorithm/cpp/1250_is_good_array/is_good_array.cpp
 */

#include <cassert>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isGoodArray(vector<int>& nums) {
    int divisor = nums[0];
    for (int num : nums) {
      divisor = gcd(divisor, num);
      if (divisor == 1) {
        break;
      }
    }
    return divisor == 1;
  }
};

int main() {
  {
    vector<int> nums{12, 5, 7, 23};
    assert(Solution().isGoodArray(nums));
  }

  {
    vector<int> nums{29, 6, 10};
    assert(Solution().isGoodArray(nums));
  }

  {
    vector<int> nums{3, 6};
    assert(!Solution().isGoodArray(nums));
  }
}