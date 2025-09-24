#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countDays(int days, vector<vector<int>>& meetings) {
    sort(meetings.begin(), meetings.end());
    int l = 1, r = 0;
    for (auto m : meetings) {
      if (m[0] > r) {
        days -= (r - l + 1);
        l = m[0];
      }
      r = max(r, m[1]);
    }
    days -= (r - l + 1);
    return days;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {10, {{5, 7}, {1, 3}, {9, 10}}, 2},
      {5, {{2, 4}, {1, 3}}, 1},
      {6, {{1, 6}}, 0},
  };

  for (auto [days, meetings, expected] : tests) {
    assert(Solution().countDays(days, meetings) == expected);
  }
}