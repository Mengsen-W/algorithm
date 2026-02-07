#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumDeletions(string s) {
    int leftb = 0, righta = 0;
    for (int i = 0; i < s.size(); i++) {
      if (s[i] == 'a') {
        righta++;
      }
    }
    int res = righta;
    for (int i = 0; i < s.size(); i++) {
      char c = s[i];
      if (c == 'a') {
        righta--;
      } else {
        leftb++;
      }
      res = min(res, leftb + righta);
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"aababbab", 2},
      {"bbaaaaabb", 2},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().minimumDeletions(s) == expected);
  }
}
