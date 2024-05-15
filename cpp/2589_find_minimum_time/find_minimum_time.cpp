/*
 * @Date: 2024-05-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-15
 * @FilePath: /algorithm/cpp/2589_find_minimum_time/find_minimum_time.cpp
 */

#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findMinimumTime(vector<vector<int>> &tasks) {
    sort(tasks.begin(), tasks.end(),
         [&](const vector<int> &t1, const vector<int> &t2) -> bool { return t1[1] < t2[1]; });
    vector<vector<int>> st;
    st.push_back({-1, -1, 0});
    for (auto &task : tasks) {
      int start = task[0], end = task[1], duration = task[2];
      int k =
          lower_bound(st.begin(), st.end(), start, [&](const vector<int> &seg, int x) -> bool { return seg[0] < x; }) -
          st.begin();
      duration -= st.back()[2] - st[k - 1][2];
      if (start <= st[k - 1][1]) {
        duration -= st[k - 1][1] - start + 1;
      }
      if (duration <= 0) {
        continue;
      }
      while (end - st.back()[1] <= duration) {
        duration += st.back()[1] - st.back()[0] + 1;
        st.pop_back();
      }
      st.push_back({end - duration + 1, end, st.back()[2] + duration});
    }
    return st.back()[2];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{2, 3, 1}, {4, 5, 1}, {1, 5, 2}}, 2},
      {{{1, 3, 2}, {2, 5, 3}, {5, 6, 2}}, 4},
  };

  for (auto &[tasks, ans] : tests) {
    assert(Solution().findMinimumTime(tasks) == ans);
  }
}