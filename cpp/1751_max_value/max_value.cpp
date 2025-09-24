#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxValue(vector<vector<int>>& events, int k) {
    int n = events.size();
    vector<vector<int>> dp(n + 1, vector<int>(k + 1));
    ranges::sort(events, [](const vector<int>& a, const vector<int>& b) { return a[1] < b[1]; });

    for (int i = 0; i < n; i++) {
      int p = ranges::lower_bound(events, events[i][0], {}, [](const auto& e) { return e[1]; }) - events.begin();
      for (int j = 1; j <= k; j++) {
        dp[i + 1][j] = max(dp[i][j], dp[p][j - 1] + events[i][2]);
      }
    }

    return dp[n][k];
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, int>> tests{
      {{{1, 2, 4}, {3, 4, 3}, {2, 3, 1}}, 2, 7},
      {{{1, 2, 4}, {3, 4, 3}, {2, 3, 10}}, 2, 10},
      {{{1, 1, 1}, {2, 2, 2}, {3, 3, 3}, {4, 4, 4}}, 3, 9},
  };

  for (auto& [events, k, ans] : tests) {
    assert(Solution().maxValue(events, k) == ans);
  }
}