/*
 * @Date: 2022-04-30 08:16:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-30 08:28:25
 * @FilePath: /algorithm/908_smallest_range_i/smallest_range_i.cpp
 */

#include <algorithm>
#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int smallestRangeI(vector<int> nums, int k) {
    int minNum = *min_element(nums.begin(), nums.end());
    int maxNum = *max_element(nums.begin(), nums.end());
    return maxNum - minNum <= 2 * k ? 0 : maxNum - minNum - 2 * k;
  }
};

int main() {
  assert(Solution().smallestRangeI({1}, 0) == 0);
  assert(Solution().smallestRangeI({0, 10}, 2) == 6);
  assert(Solution().smallestRangeI({1, 3, 6}, 3) == 0);
}