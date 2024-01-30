/*
 * @Date: 2024-01-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-30
 * @FilePath: /algorithm/cpp/2808_minimum_seconds/minimum_seconds.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumSeconds(vector<int>& nums) {
    unordered_map<int, vector<int>> mp;
    int n = nums.size(), res = n;
    for (int i = 0; i < n; ++i) {
      mp[nums[i]].push_back(i);
    }
    for (auto& pos : mp) {
      int mx = pos.second[0] + n - pos.second.back();
      for (int i = 1; i < pos.second.size(); ++i) {
        mx = max(mx, pos.second[i] - pos.second[i - 1]);
      }
      res = min(res, mx / 2);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 1, 2}, 1},
      {{2, 1, 3, 3, 2}, 2},
      {{5, 5, 5, 5}, 0},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().minimumSeconds(nums) == ans);
  }
}