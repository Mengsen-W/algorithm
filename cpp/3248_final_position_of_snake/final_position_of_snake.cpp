#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int finalPositionOfSnake(int n, vector<string>& commands) {
    int ans = 0;
    for (const string& c : commands) {
      if (c[0] == 'U') {
        ans -= n;
      } else if (c[0] == 'D') {
        ans += n;
      } else if (c[0] == 'L') {
        --ans;
      } else {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<string>, int>> tests{
      {2, {"RIGHT", "DOWN"}, 3},
      {3, {"DOWN", "RIGHT", "UP"}, 1},
  };

  for (auto &[n, commands, ans] : tests) {
    assert(Solution().finalPositionOfSnake(n, commands) == ans);
  }
}