#include <cassert>
#include <stack>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string reverseParentheses(string s) {
    int n = s.length();
    vector<int> pair(n);
    stack<int> stk;
    for (int i = 0; i < n; i++) {
      if (s[i] == '(') {
        stk.push(i);
      } else if (s[i] == ')') {
        int j = stk.top();
        stk.pop();
        pair[i] = j, pair[j] = i;
      }
    }

    string ret;
    int index = 0, step = 1;
    while (index < n) {
      if (s[index] == '(' || s[index] == ')') {
        index = pair[index];
        step = -step;
      } else {
        ret.push_back(s[index]);
      }
      index += step;
    }
    return ret;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"(abcd)", "dcba"},
      {"(u(love)i)", "iloveu"},
      {"(ed(et(oc))el)", "leetcode"},
      {"a(bcdefghijkl(mno)p)q", "apmnolkjihgfedcbq"},
  };

  for (auto& [s, ans] : tests) {
    assert(Solution().reverseParentheses(s) == ans);
  }
  return 0;
}