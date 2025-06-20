#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxDistance(string s, int k) {
    int ans = 0;
    int north = 0, south = 0, east = 0, west = 0;
    for (char it : s) {
      switch (it) {
        case 'N':
          north++;
          break;
        case 'S':
          south++;
          break;
        case 'E':
          east++;
          break;
        case 'W':
          west++;
          break;
      }
      int times1 = min({north, south, k});         // modification times for N and S
      int times2 = min({east, west, k - times1});  // modification times for E and W
      ans = max(ans, count(north, south, times1) + count(east, west, times2));
    }
    return ans;
  }

  int count(int drt1, int drt2, int times) {
    return abs(drt1 - drt2) + times * 2;
  }  // Calculate modified Manhattan distance
};

int main() {
  vector<tuple<string, int, int>> tests{
      {"NWSE", 1, 3},
      {"NSWWEW", 3, 6},
  };

  for (auto& [s, k, ans] : tests) {
    assert(Solution().maxDistance(s, k) == ans);
  }
}