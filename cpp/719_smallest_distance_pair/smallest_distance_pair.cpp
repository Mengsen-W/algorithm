/*
 * @Date: 2022-06-16 09:46:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-16 09:50:34
 * @FilePath: /algorithm/719_smallest_distance_pair/smallest_distance_pair.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int smallestDistancePair(vector<int> nums, int k) {
    sort(nums.begin(), nums.end());
    int n = nums.size(), left = 0, right = nums.back() - nums.front();
    while (left <= right) {
      int mid = (left + right) / 2;
      int cnt = 0;
      for (int i = 0, j = 0; j < n; j++) {
        while (nums[j] - nums[i] > mid) {
          i++;
        }
        cnt += j - i;
      }
      if (cnt >= k) {
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }
    return left;
  }
};

int main() {
  assert(Solution().smallestDistancePair({1, 3, 1}, 1) == 0);
  assert(Solution().smallestDistancePair({1, 1, 1}, 2) == 0);
  assert(Solution().smallestDistancePair({1, 6, 1}, 3) == 5);
}