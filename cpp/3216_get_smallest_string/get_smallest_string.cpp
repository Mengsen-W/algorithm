#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string getSmallestString(string s) {
    for (int i = 0; i + 1 < s.size(); i++) {
      if (s[i] > s[i + 1] && s[i] % 2 == s[i + 1] % 2) {
        swap(s[i], s[i + 1]);
        break;
      }
    }
    return s;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"45320", "43520"},
      {"001", "001"},
  };

  for (auto&[s, ans] : tests) {
    assert(Solution().getSmallestString(s) == ans);
  }
}