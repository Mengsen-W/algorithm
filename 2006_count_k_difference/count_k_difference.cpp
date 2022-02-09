/*
 * @Date: 2022-02-08 23:56:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-09 00:15:44
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int countKDifference(vector<int> nums, int k) {
    int res = 0, n = nums.size();
    unordered_map<int, int> cnt;
    for (int j = 0; j < n; ++j) {
      res += (cnt.count(nums[j] - k) ? cnt[nums[j] - k] : 0);
      res += (cnt.count(nums[j] + k) ? cnt[nums[j] + k] : 0);
      ++cnt[nums[j]];
    }
    return res;
  }
};

int main() {
  assert(Solution().countKDifference({1, 2, 2, 1}, 1) == 4);
  assert(Solution().countKDifference({1, 3}, 3) == 0);
  assert(Solution().countKDifference({3, 2, 1, 5, 4}, 2) == 3);
}
