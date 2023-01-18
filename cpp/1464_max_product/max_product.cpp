/*
 * @Date: 2022-08-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-26
 * @FilePath: /algorithm/1464_max_product/max_product.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxProduct(vector<int>& nums) {
    int a = nums[0], b = nums[1];
    if (a < b) {
      swap(a, b);
    }
    for (int i = 2; i < nums.size(); i++) {
      if (nums[i] > a) {
        b = a;
        a = nums[i];
      } else if (nums[i] > b) {
        b = nums[i];
      }
    }
    return (a - 1) * (b - 1);
  }
};

int main() {
  {
    vector<int> nums{3, 4, 5, 2};
    int ans = 12;
    assert(Solution().maxProduct(nums) == ans);
  }
  {
    vector<int> nums{1, 5, 4, 5};
    int ans = 16;
    assert(Solution().maxProduct(nums) == ans);
  }
  {
    vector<int> nums{3, 7};
    int ans = 12;
    assert(Solution().maxProduct(nums) == ans);
  }
}