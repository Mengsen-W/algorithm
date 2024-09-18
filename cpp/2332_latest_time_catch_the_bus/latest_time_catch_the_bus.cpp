#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int latestTimeCatchTheBus(vector<int>& buses, vector<int>& passengers, int capacity) {
    sort(buses.begin(), buses.end());
    sort(passengers.begin(), passengers.end());
    int pos = 0;
    int space = 0;
    for (int arrive : buses) {
      space = capacity;
      while (space > 0 && pos < passengers.size() && passengers[pos] <= arrive) {
        space--;
        pos++;
      }
    }

    pos--;
    int lastCatchTime = space > 0 ? buses.back() : passengers[pos];
    while (pos >= 0 && passengers[pos] == lastCatchTime) {
      pos--;
      lastCatchTime--;
    }

    return lastCatchTime;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, int>> tests{
      {{10, 20}, {2, 17, 18, 19}, 2, 16},
      {{20, 30, 10}, {19, 13, 26, 4, 25, 11, 21}, 2, 20},
  };

  for (auto& [buses, passengers, capacity, ans] : tests) {
    assert(Solution().latestTimeCatchTheBus(buses, passengers, capacity) == ans);
  }
}