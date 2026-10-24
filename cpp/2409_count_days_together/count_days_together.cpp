#include <cassert>
#include <string>
#include <tuple>
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

  int calculateDayOfYear(string day, const vector<int>& prefixSum) {
    int month = stoi(day.substr(0, 2));
    int date = stoi(day.substr(3));
    return prefixSum[month - 1] + date;
  }
};

int main() {
  vector<tuple<string, string, string, string, int>> tests{
      {"08-15", "08-18", "08-16", "08-19", 3},
      {"10-01", "10-31", "11-01", "12-31", 0},
  };

  for (auto& [arriveAlice, leaveAlice, arriveBob, leaveBob, ans] : tests) {
    assert(Solution().countDaysTogether(arriveAlice, leaveAlice, arriveBob, leaveBob) == ans);
  }
}