#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool squareIsWhite(string coordinates) { return ((coordinates[0] - 'a' + 1) + (coordinates[1] - '0')) % 2 == 1; }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"a1", false},
      {"h3", true},
      {"c7", false},
  };

  for (auto &[coordinates, ans] : tests) {
    assert(Solution().squareIsWhite(coordinates) == ans);
  }
}