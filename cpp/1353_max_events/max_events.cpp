#include <algorithm>
#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxEvents(vector<vector<int>>& events) {
    int n = events.size();
    int maxDay = 0;
    for (int i = 0; i < events.size(); i++) {
      maxDay = max(maxDay, events[i][1]);
    }
    priority_queue<int, vector<int>, greater<>> pq;
    sort(events.begin(), events.end());
    int ans = 0;
    for (int i = 0, j = 0; i <= maxDay; i++) {
      while (j < n && events[j][0] <= i) {
        pq.emplace(events[j][1]);
        j++;
      }
      while (!pq.empty() && pq.top() < i) {
        pq.pop();
      }
      if (!pq.empty()) {
        pq.pop();
        ans++;
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 2}, {2, 3}, {3, 4}}, 3},
      {{{1, 2}, {2, 3}, {3, 4}, {1, 2}}, 4},
  };

  for (auto& [events, ans] : tests) {
    assert(Solution().maxEvents(events) == ans);
  }
}