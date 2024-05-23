/*
 * @Date: 2024-05-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-23
 * @FilePath: /algorithm/cpp/2831_longest_equal_subarray/longest_equal_subarray.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int longestEqualSubarray(vector<int>& nums, int k) {
    int n = nums.size();
    unordered_map<int, vector<int>> pos;
    for (int i = 0; i < n; i++) {
      pos[nums[i]].emplace_back(i);
    }
    int ans = 0;
    for (auto& [_, vec] : pos) {
      /* 缩小窗口，直到不同元素数量小于等于 k */
      for (int i = 0, j = 0; i < vec.size(); i++) {
        while (vec[i] - vec[j] - (i - j) > k) {
          j++;
        }
        ans = max(ans, i - j + 1);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{1, 3, 2, 3, 1, 3}, 3, 3},
      {{1, 1, 2, 2, 1, 1}, 2, 4},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().longestEqualSubarray(nums, k) == ans);
  }
}