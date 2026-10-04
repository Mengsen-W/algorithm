#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int scoreOfParentheses(string s) {
    int bal = 0, n = s.size(), res = 0;
    for (int i = 0; i < n; i++) {
      bal += (s[i] == '(' ? 1 : -1);
      if (s[i] == ')' && s[i - 1] == '(') {
        res += 1 << bal;
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"()", 1},
      {"(())", 2},
      {"()()", 2},
      {"(()(()))", 6},
  };

  for (auto [s, expected] : tests) {
    assert(Solution().scoreOfParentheses(s) == expected);
  }
}