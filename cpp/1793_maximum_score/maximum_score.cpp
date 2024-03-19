/*
 * @Date: 2024-03-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-19
 * @FilePath: /algorithm/cpp/1793_maximum_score/maximum_score.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maximumScore(vector<int>& nums, int k) {
    int n = nums.size();
    int left = k - 1, right = k + 1;
    int ans = 0;
    for (int i = nums[k];;) {
      while (left >= 0 && nums[left] >= i) {
        --left;
      }
      while (right < n && nums[right] >= i) {
        ++right;
      }
      ans = max(ans, (right - left - 1) * i);
      if (left == -1 && right == n) {
        break;
      }
      i = max((left == -1 ? -1 : nums[left]), (right == n ? -1 : nums[right]));
      if (i == -1) {
        break;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 4, 3, 7, 4, 5}, 3, 15},
      {{5, 5, 4, 5, 4, 1, 1, 1}, 0, 20},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().maximumScore(nums, k) == ans);
  }
}