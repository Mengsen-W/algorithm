/*
 * @Date: 2023-08-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-28
 * @FilePath: /algorithm/cpp/57_insert/insert.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
    int left = newInterval[0];
    int right = newInterval[1];
    bool placed = false;  // 确定 newInterval 是否插入
    vector<vector<int>> ans;
    for (const auto& interval : intervals) {
      if (interval[0] > right) {
        // 在插入区间的右侧且无交集
        if (!placed) {
          ans.push_back({left, right});
          placed = true;
        }
        ans.push_back(interval);
      } else if (interval[1] < left) {
        // 在插入区间的左侧且无交集
        ans.push_back(interval);
      } else {
        // 与插入区间有交集，计算它们的并集，并更新
        left = min(left, interval[0]);
        right = max(right, interval[1]);
      }
    }
    if (!placed) {
      ans.push_back({left, right});
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>, vector<vector<int>>>> tests{
      {{{1, 3}, {6, 9}}, {2, 5}, {{1, 5}, {6, 9}}},
      {{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, {4, 8}, {{1, 2}, {3, 10}, {12, 16}}},
      {{}, {5, 7}, {{5, 7}}},
      {{{1, 5}}, {2, 3}, {{1, 5}}},
      {{{1, 5}}, {2, 7}, {{1, 7}}},
  };

  for (auto& [intervals, newInterval, ans] : tests) {
    assert(Solution().insert(intervals, newInterval) == ans);
  }
}