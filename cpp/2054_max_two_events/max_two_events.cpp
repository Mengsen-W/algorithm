#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

struct Event {
  // 时间戳
  int ts;
  // op = 0 表示左边界，op = 1 表示右边界
  int op;
  int val;
  Event(int _ts, int _op, int _val) : ts(_ts), op(_op), val(_val) {}
  bool operator<(const Event& that) const { return tie(ts, op) < tie(that.ts, that.op); }
};

class Solution {
 public:
  int maxTwoEvents(vector<vector<int>>& events) {
    vector<Event> evs;
    for (const auto& event : events) {
      evs.emplace_back(event[0], 0, event[2]);
      evs.emplace_back(event[1], 1, event[2]);
    }
    sort(evs.begin(), evs.end());

    int ans = 0, bestFirst = 0;
    for (const auto& [ts, op, val] : evs) {
      if (op == 0) {
        ans = max(ans, val + bestFirst);
      } else {
        bestFirst = max(bestFirst, val);
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int>> tests{
      {{{1, 3, 2}, {4, 5, 2}, {2, 4, 3}}, 4},
      {{{1, 3, 2}, {4, 5, 2}, {1, 5, 5}}, 5},
      {{{1, 5, 3}, {1, 5, 1}, {6, 6, 5}}, 8},
  };

  for (auto& [events, expect] : tests) {
    assert(Solution().maxTwoEvents(events) == expect);
  }
}
