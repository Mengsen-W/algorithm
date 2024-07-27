#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string getSmallestString(string s, int k) {
    for (int i = 0; i < s.size(); ++i) {
      int dis = min(s[i] - 'a', 'z' - s[i] + 1);
      if (dis <= k) {
        s[i] = 'a';
        k -= dis;
      } else {
        s[i] -= k;
        break;
      }
    }
    return s;
  }
};

int main() {
  vector<tuple<string, int, string>> tests{
      {"zbbz", 3, "aaaz"},
      {"xaxcd", 4, "aawcd"},
      {"lol", 0, "lol"},
  };

  for (auto &[s, k, ans] : tests) {
    assert(Solution().getSmallestString(s, k) == ans);
  }
}