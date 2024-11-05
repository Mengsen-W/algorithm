#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string losingPlayer(int x, int y) { return min(x, y / 4) % 2 ? "Alice" : "Bob"; }
};

int main() {
  vector<tuple<int, int, string>> tests{
      {2, 7, "Alice"},
      {4, 11, "Bob"},
  };

  for (auto &[x, y, ans] : tests) {
    assert(Solution().losingPlayer(x, y) == ans);
  }
}