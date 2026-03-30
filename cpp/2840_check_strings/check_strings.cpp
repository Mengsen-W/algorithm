#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool checkStrings(string s1, string s2) {
    if (s1.length() != s2.length()) {
      return false;
    }

    int counts[256] = {0};

    for (int i = 0; i < s1.length(); i++) {
      int offset = (i & 1) << 7;
      counts[offset + s1[i]]++;
      counts[offset + s2[i]]--;
    }

    for (int i = 0; i < 256; i++) {
      if (counts[i] != 0) {
        return false;
      }
    }

    return true;
  }
};

int main() {
  vector<tuple<string, string, bool>> tests{
      {"abcdba", "cabdab", true},
      {"abe", "bea", false},
  };

  for (auto& [s1, s2, expected] : tests) {
    assert(Solution().checkStrings(s1, s2) == expected);
  }
}