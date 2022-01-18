/*
 * @Date: 2022-01-18 02:13:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-18 03:09:18
 */

#include <cassert>
#include <climits>
#include <string>
#include <vector>

using namespace std;

class Solution {
  int getMinutes(string &t) {
    return (int(t[0] - '0') * 10 + int(t[1] - '0')) * 60 +
           int(t[3] - '0') * 10 + int(t[4] - '0');
  }

 public:
  int findMinDifference(vector<string> timePoints) {
    int n = timePoints.size();
    if (n > 1440) {
      return 0;
    }
    sort(timePoints.begin(), timePoints.end());
    int ans = INT_MAX;
    int t0Minutes = getMinutes(timePoints[0]);
    int preMinutes = t0Minutes;
    for (int i = 1; i < n; ++i) {
      int minutes = getMinutes(timePoints[i]);
      ans = min(ans, minutes - preMinutes);  // 相邻时间的时间差
      preMinutes = minutes;
    }
    ans = min(ans, t0Minutes + 1440 - preMinutes);  // 首尾时间的时间差
    return ans;
  }
};

int main() {
  assert(Solution().findMinDifference({"23:59", "00:00"}) == 1);
  assert(Solution().findMinDifference({"00:00", "23:59", "00:00"}) == 0);
}