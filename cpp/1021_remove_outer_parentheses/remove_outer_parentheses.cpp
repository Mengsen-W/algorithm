#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string removeOuterParentheses(string s) {
    int level = 0;
    string res;
    for (auto c : s) {
      if (c == ')') {
        level--;
      }
      if (level) {
        res.push_back(c);
      }
      if (c == '(') {
        level++;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"(()())(())", "()()()"},
      {"(()())(())(()(()))", "()()()()(())"},
      {"()()", ""},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().removeOuterParentheses(s) == expected);
  }
  return 0;
}
