#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkTwoChessboards(string coordinate1, string coordinate2) {
    return (coordinate1[0] - coordinate2[0] + coordinate1[1] - coordinate2[1]) % 2 == 0;
  }
};

int main() {
  vector<tuple<string, string, bool>> tests{
      {"a1", "c3", true},
      {"a1", "h3", false},
  };

  for (auto &[coordinate1, coordinate2, ans] : tests) {
    assert(Solution().checkTwoChessboards(coordinate1, coordinate2) == ans);
  }
}