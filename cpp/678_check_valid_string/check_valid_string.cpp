#include <cassert>
#include <cmath>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkValidString(string s) {
    int minCount = 0, maxCount = 0;
    int n = s.size();
    for (int i = 0; i < n; i++) {
      char c = s[i];
      if (c == '(') {
        minCount++;
        maxCount++;
      } else if (c == ')') {
        minCount = max(minCount - 1, 0);
        maxCount--;
        if (maxCount < 0) return false;
      } else {
        minCount = max(minCount - 1, 0);
        maxCount++;
      }
    }
    return minCount == 0;
  }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"()", true},
      {"(*)", true},
      {"(*))", true},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().checkValidString(s) == expected);
  }
  return 0;
}
