#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int removeCoveredIntervals(vector<vector<int>>& intervals) {
    int n = intervals.size();
    sort(intervals.begin(), intervals.end(),
         [](const vector<int>& u, const vector<int>& v) { return u[0] < v[0] || (u[0] == v[0] && u[1] > v[1]); });
    int ans = n;
    int rmax = intervals[0][1];
    for (int i = 1; i < n; ++i) {
      if (intervals[i][1] <= rmax) {
        --ans;
      } else {
        rmax = max(rmax, intervals[i][1]);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 4}, {3, 6}, {2, 8}}, 2},
  };

  for (auto& [intervals, ans] : tests) {
    assert(Solution().removeCoveredIntervals(intervals) == ans);
  }
}