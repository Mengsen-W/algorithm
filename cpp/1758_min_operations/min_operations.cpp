#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(string s) {
    int cnt = 0;
    for (int i = 0; i < s.size(); i++) {
      char c = s[i];
      if (c != ('0' + i % 2)) {
        cnt++;
      }
    }
    return min(cnt, (int)s.size() - cnt);
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"0100", 1},
      {"10", 0},
      {"1111", 2},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().minOperations(s) == expected);
  }
}
