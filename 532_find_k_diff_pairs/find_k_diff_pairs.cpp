/*
 * @Date: 2022-06-16 09:53:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-16 10:10:34
 * @FilePath: /algorithm/532_find_k_diff_pairs/find_k_diff_pairs.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int findPairs(vector<int> nums, int k) {
    sort(nums.begin(), nums.end());
    int n = nums.size(), y = 0, res = 0;
    for (int x = 0; x < n; x++) {
      if (x == 0 || nums[x] != nums[x - 1]) {
        while (y < n && (nums[y] < nums[x] + k || y <= x)) {
          y++;
        }
        if (y < n && nums[y] == nums[x] + k) {
          res++;
        }
      }
    }
    return res;
  }
};

int main() {
  assert(Solution().findPairs({3, 1, 4, 1, 5}, 2) == 2);
  assert(Solution().findPairs({1, 2, 3, 4, 5}, 1) == 4);
  assert(Solution().findPairs({1, 3, 1, 5, 4}, 0) == 1);
}