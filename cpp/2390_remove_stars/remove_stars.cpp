#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string removeStars(string s) {
    string res;
    for (char c : s) {
      if (c == '*') {
        res.pop_back();
      } else {
        res += c;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"leet**cod*e", "lecoe"},
      {"erase*****", ""},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().removeStars(s) == ans);
  }
}