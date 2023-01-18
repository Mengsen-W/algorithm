/*
 * @Date: 2022-02-14 08:26:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-14 08:58:38
 * @FilePath: /algorithm/540_single_non_duplicate/single_non_duplicate.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int singleNonDuplicate(vector<int> nums) {
    int low = 0, high = nums.size() - 1;
    while (low < high) {
      int mid = (high - low) / 2 + low;
      mid -= mid & 1;
      if (nums[mid] == nums[mid + 1]) {
        low = mid + 2;
      } else {
        high = mid;
      }
    }
    return nums[low];
  }
};

int main() {
  assert(Solution().singleNonDuplicate({1, 1, 2, 3, 3, 4, 4, 8, 8}) == 2);
  assert(Solution().singleNonDuplicate({3, 3, 7, 7, 10, 11, 11}) == 10);
}