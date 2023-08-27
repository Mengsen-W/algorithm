/*
 * @Date: 2023-08-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-27
 * @FilePath: /algorithm/cpp/56_merge/merge.cpp
 */

#include <algorithm>
#include <cassert>
#include <iostream>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> merge(vector<vector<int>>& intervals) {
    sort(intervals.begin(), intervals.end());
    vector<vector<int>> res{};
    for (size_t i = 0; i < intervals.size(); ++i) {
      int left = intervals[i][0], right = intervals[i][1];
      if (res.empty() || res.back()[1] < left) {
        res.push_back({left, right});
      } else {
        res.back()[1] = max(res.back()[1], right);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>>> tests{
      {{{1, 3}, {2, 6}, {8, 10}, {15, 18}}, {{1, 6}, {8, 10}, {15, 18}}},
      {{{1, 4}, {4, 5}}, {{1, 5}}},
      {{{1, 4}, {0, 4}}, {{0, 4}}},
  };

  for (auto& [intervals, res] : tests) {
    assert(Solution().merge(intervals) == res);
  }
}