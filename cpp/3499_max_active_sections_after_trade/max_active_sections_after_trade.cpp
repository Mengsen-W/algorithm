#include <algorithm>
#include <cassert>
#include <climits>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int maxActiveSectionsAfterTrade(string s) {
    int n = s.size();
    int cnt1 = count(s.begin(), s.end(), '1');

    int i = 0;
    int bestGain = 0;
    int prev = INT_MIN, cur = 0;

    while (i < n) {
      int start = i;

      while (i < n && s[i] == s[start]) {
        ++i;
      }
      if (s[start] == '0') {
        cur = i - start;
        bestGain = max(bestGain, prev + cur);
        prev = cur;
      }
    }

    return cnt1 + bestGain;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"01", 1},
      {"0100", 4},
      {"1000100", 7},
      {"01010", 4},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().maxActiveSectionsAfterTrade(s) == expected);
  }
}