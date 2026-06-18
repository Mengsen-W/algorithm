#include <algorithm>
#include <cassert>
#include <cstdlib>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  double angleClock(int hour, int minutes) {
    int oneMinAngle = 6;
    int oneHourAngle = 30;

    double minutesAngle = oneMinAngle * minutes;
    double hourAngle = (hour % 12 + minutes / 60.0) * oneHourAngle;

    double diff = abs(hourAngle - minutesAngle);
    return min(diff, 360 - diff);
  }
};

int main() {
  vector<tuple<int, int, double>> tests{
      {12, 30, 165},
      {3, 30, 75},
      {3, 15, 7.5},
      {4, 50, 155},
      {12, 0, 0},
  };

  for (auto [hour, minutes, expected] : tests) {
    assert(Solution().angleClock(hour, minutes) == expected);
  }

  return 0;
}