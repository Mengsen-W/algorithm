/*
 * @Date: 2024-05-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-24
 * @FilePath: /algorithm/cpp/1673_most_competitive/most_competitive.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> mostCompetitive(vector<int>& nums, int k) {
    vector<int> res;
    int n = nums.size();
    for (int i = 0; i < n; i++) {
      while (!res.empty() && n - i + res.size() > k && res.back() > nums[i]) {
        res.pop_back();
      }
      res.push_back(nums[i]);
    }
    res.resize(k);
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<int>>> tests{
      {{3, 5, 2, 6}, 2, {2, 6}},
      {{2, 4, 3, 3, 5, 4, 9, 6}, 4, {2, 3, 3, 4}},
  };

  for (auto& [nums, k, ans] : tests) {
    assert(Solution().mostCompetitive(nums, k) == ans);
  }
}