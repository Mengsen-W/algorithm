/*
 * @Date: 2022-04-28 09:24:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-28 09:34:54
 * @FilePath: /algorithm/905_sort_array_by_parity/sort_array_by_parity.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> sortArrayByParity(vector<int> nums) {
    int left = 0, right = nums.size() - 1;
    while (left < right) {
      while (left < right && nums[left] % 2 == 0) {
        left++;
      }
      while (left < right && nums[right] % 2 == 1) {
        right--;
      }
      if (left < right) {
        swap(nums[left++], nums[right--]);
      }
    }
    return nums;
  }
};

int main() {
  assert((Solution().sortArrayByParity(vector<int>{3, 1, 2, 4}) == vector<int>{4, 2, 1, 3}));
  assert(Solution().sortArrayByParity(vector<int>{0}) == vector<int>{0});
}
