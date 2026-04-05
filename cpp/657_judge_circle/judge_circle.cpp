#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  bool judgeCircle(string moves) {
    int x = 0, y = 0;
    for (const auto& move : moves) {
      if (move == 'U') {
        y--;
      } else if (move == 'D') {
        y++;
      } else if (move == 'L') {
        x--;
      } else if (move == 'R') {
        x++;
      }
    }
    return x == 0 && y == 0;
  }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"UD", true},
      {"LL", false},
  };

  for (auto& test : tests) {
    assert(Solution().judgeCircle(get<0>(test)) == get<1>(test));
  }
}