#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxFreeTime(int eventTime, vector<int>& startTime, vector<int>& endTime) {
    int n = startTime.size(), res = 0;
    for (int i = 0, t1 = 0, t2 = 0; i < n; i++) {
      int left1 = i == 0 ? 0 : endTime[i - 1];
      int right1 = i == n - 1 ? eventTime : startTime[i + 1];
      if (endTime[i] - startTime[i] <= t1) {
        res = max(res, right1 - left1);
      }
      t1 = max(t1, startTime[i] - (i == 0 ? 0 : endTime[i - 1]));

      res = max(res, right1 - left1 - (endTime[i] - startTime[i]));

      int left2 = i == n - 1 ? 0 : endTime[n - i - 2];
      int right2 = i == 0 ? eventTime : startTime[n - i];
      if (endTime[n - i - 1] - startTime[n - i - 1] <= t2) {
        res = max(res, right2 - left2);
      }
      t2 = max(t2, (i == 0 ? eventTime : startTime[n - i]) - endTime[n - i - 1]);
    }
    return res;
  }
};

int main() {
  vector<tuple<int, vector<int>, vector<int>, int>> tests{
      {5, {1, 3}, {2, 5}, 2},
      {10, {0, 7, 9}, {1, 8, 10}, 7},
      {10, {0, 3, 7, 9}, {1, 4, 8, 10}, 6},
      {5, {0, 1, 2, 3, 4}, {1, 2, 3, 4, 5}, 0},
  };

  for (auto& [eventTime, startTime, endTime, ans] : tests) {
    assert(Solution().maxFreeTime(eventTime, startTime, endTime) == ans);
  }
}