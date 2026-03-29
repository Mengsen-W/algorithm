#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool canBeEqual(string s1, string s2) {
    if (s1 == s2) {
      return true;
    } else if (s1[0] == s2[2] && s1[2] == s2[0] && s1[1] == s2[3] && s1[3] == s2[1]) {
      return true;
    } else if (s1[0] == s2[0] && s1[2] == s2[2] && s1[1] == s2[3] && s1[3] == s2[1]) {
      return true;
    } else if (s1[1] == s2[1] && s1[3] == s2[3] && s1[0] == s2[2] && s1[2] == s2[0]) {
      return true;
    }
    return false;
  }
};

int main() {
  vector<tuple<string, string, bool>> tests{
      {"abcd", "cdab", true},
      {"abcd", "dacb", false},
  };

  for (auto [s1, s2, expected] : tests) {
    assert(Solution().canBeEqual(s1, s2) == expected);
  }
}