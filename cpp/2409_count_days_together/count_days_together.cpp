/*
 * @Date: 2023-04-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-17
 * @FilePath: /algorithm/cpp/2409_count_days_together/count_days_together.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int countDaysTogether(string arriveAlice, string leaveAlice, string arriveBob, string leaveBob) {
    vector<int> datesOfMonths = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    vector<int> prefixSum(1, 0);
    for (int date : datesOfMonths) {
      prefixSum.emplace_back(prefixSum.back() + date);
    }

    int arriveAliceDay = calculateDayOfYear(arriveAlice, prefixSum);
    int leaveAliceDay = calculateDayOfYear(leaveAlice, prefixSum);
    int arriveBobDay = calculateDayOfYear(arriveBob, prefixSum);
    int leaveBobDay = calculateDayOfYear(leaveBob, prefixSum);
    return max(0, min(leaveAliceDay, leaveBobDay) - max(arriveAliceDay, arriveBobDay) + 1);
  }

  int calculateDayOfYear(string day, const vector<int> &prefixSum) {
    int month = stoi(day.substr(0, 2));
    int date = stoi(day.substr(3));
    return prefixSum[month - 1] + date;
  }
};

int main() {
  {
    string arriveAlice = "08-15", leaveAlice = "08-18", arriveBob = "08-16", leaveBob = "08-19";
    int ans = 3;
    assert(Solution().countDaysTogether(arriveAlice, leaveAlice, arriveBob, leaveBob) == ans);
  }

  {
    string arriveAlice = "10-01", leaveAlice = "10-31", arriveBob = "11-01", leaveBob = "12-31";
    int ans = 0;
    assert(Solution().countDaysTogether(arriveAlice, leaveAlice, arriveBob, leaveBob) == ans);
  }
}