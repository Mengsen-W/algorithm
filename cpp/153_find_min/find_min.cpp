/*
 * @Date: 2021-04-08 10:15:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-08 10:32:49
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findMin(vector<int>& nums) {
    int low = 0;
    int high = nums.size() - 1;
    while (low < high) {
      int pivot = low + (high - low) / 2;
      if (nums[pivot] < nums[high]) {
        high = pivot;
      } else {
        low = pivot + 1;
      }
    }
    return nums[low];
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 4, 5, 1, 2}, 1},
      {{4, 5, 6, 7, 0, 1, 2}, 0},
      {{11, 13, 15, 17}, 11},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().findMin(nums) == expected);
  }
}