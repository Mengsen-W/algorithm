/*
 * @Date: 2023-07-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-18
 * @FilePath: /algorithm/cpp/1851_min_interval/min_interval.cpp
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> minInterval(vector<vector<int>>& intervals, vector<int>& queries) {
    int n = intervals.size(), m = queries.size();
    sort(intervals.begin(), intervals.end());
    using pii = pair<int, int>;
    vector<pii> qs;
    for (int i = 0; i < m; ++i) {
      qs.emplace_back(queries[i], i);
    }
    sort(qs.begin(), qs.end());
    vector<int> ans(m, -1);
    priority_queue<pii, vector<pii>, greater<pii>> pq;
    int i = 0;
    for (auto& [x, j] : qs) {
      while (i < n && intervals[i][0] <= x) {
        int a = intervals[i][0], b = intervals[i][1];
        pq.emplace(b - a + 1, b);
        ++i;
      }
      while (!pq.empty() && pq.top().second < x) {
        pq.pop();
      }
      if (!pq.empty()) {
        ans[j] = pq.top().first;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>, vector<int>>> test_cases{
      {{{1, 4}, {2, 4}, {3, 6}, {4, 4}}, {2, 3, 4, 5}, {3, 3, 1, 4}},
      {{{2, 3}, {2, 5}, {1, 8}, {20, 25}}, {2, 19, 5, 22}, {2, -1, 4, 6}},
  };

  for (auto& [intervals, queries, ans] : test_cases) {
    assert(Solution().minInterval(intervals, queries) == ans);
  }
}