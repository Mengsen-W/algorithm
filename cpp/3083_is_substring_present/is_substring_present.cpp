#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isSubstringPresent(string s) {
    vector<int> h(26);
    for (int i = 0; i + 1 < s.size(); i++) {
      int x = s[i] - 'a';
      int y = s[i + 1] - 'a';
      h[x] |= 1 << y;
      if (h[y] >> x & 1) {
        return true;
      }
    }
    return false;
  }
};

int main() {
  vector<tuple<string, bool>> tests{
      {"leetcode", true},
      {"abcba", true},
      {"abcd", false},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().isSubstringPresent(s) == ans);
  }
}