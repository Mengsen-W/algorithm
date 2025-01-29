/*
 * @Date: 2022-01-19 01:47:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-19 01:52:56
 */

#include <cassert>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  bool containsNearbyDuplicate(vector<int> nums, int k) {
    unordered_set<int> s;
    int length = nums.size();
    for (int i = 0; i < length; i++) {
      if (i > k) {
        s.erase(nums[i - k - 1]);
      }
      if (s.count(nums[i])) {
        return true;
      }
      s.emplace(nums[i]);
    }
    return false;
  }
};

int main() {
  vector<tuple<vector<int>, int, bool>> tests{
      {{1, 2, 3, 1}, 3, true},
      {{1, 0, 1, 1}, 1, true},
      {{1, 2, 3, 1, 2, 3}, 2, false},
  };
  

  for (auto &[nums, k, ans] : tests) {
    assert(Solution().containsNearbyDuplicate(nums, k) == ans);
  }
}