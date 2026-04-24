#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int furthestDistanceFromOrigin(string moves) {
    int L = 0, R = 0, B = 0;
    for (auto c : moves) {
      if (c == 'L') {
        L++;
      } else if (c == 'R') {
        R++;
      } else {
        B++;
      }
    }
    return abs(L - R) + B;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"L_RL__R", 3},
      {"_R__LL_", 5},
      {"_______", 7},
  };

  for (auto& [moves, ans] : tests) {
    assert(Solution().furthestDistanceFromOrigin(moves) == ans);
  }
}
