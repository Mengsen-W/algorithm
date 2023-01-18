/*
 * @Date: 2022-05-20 22:19:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-20 23:12:36
 * @FilePath: /algorithm/436_find_right_interval/find_right_interval.cpp
 */

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findRightInterval(vector<vector<int>> intervals) {
    vector<pair<int, int>> startIntervals;
    vector<pair<int, int>> endIntervals;
    int n = intervals.size();
    for (int i = 0; i < n; i++) {
      startIntervals.emplace_back(intervals[i][0], i);
      endIntervals.emplace_back(intervals[i][1], i);
    }
    sort(startIntervals.begin(), startIntervals.end());
    sort(endIntervals.begin(), endIntervals.end());

    vector<int> ans(n, -1);
    for (int i = 0, j = 0; i < n && j < n; i++) {
      while (j < n && endIntervals[i].first > startIntervals[j].first) {
        j++;
      }
      if (j < n) {
        ans[endIntervals[i].second] = startIntervals[j].second;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().findRightInterval(vector<vector<int>>{{1, 2}}) == vector<int>{-1});
  assert(Solution().findRightInterval(vector<vector<int>>({{3, 4}, {2, 3}, {1, 2}})) == vector<int>({-1, 0, 1}));
  assert(Solution().findRightInterval(vector<vector<int>>({{1, 4}, {2, 3}, {3, 4}})) == vector<int>({-1, 2, -1}));
}