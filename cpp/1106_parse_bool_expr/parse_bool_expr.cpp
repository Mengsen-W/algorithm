/*
 * @Date: 2022-11-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-05
 * @FilePath: /algorithm/1106_parse_bool_expr/parse_bool_expr.cpp
 */

#include <cassert>
#include <stack>
#include <string>

using namespace std;

class Solution {
 public:
  bool parseBoolExpr(string expression) {
    stack<char> stk;
    int n = expression.size();
    for (int i = 0; i < n; i++) {
      char c = expression[i];
      if (c == ',') {
        continue;
      } else if (c != ')') {
        stk.push(c);
      } else {
        int t = 0, f = 0;
        while (stk.top() != '(') {
          char val = stk.top();
          stk.pop();
          if (val == 't') {
            t++;
          } else {
            f++;
          }
        }
        stk.pop();
        char op = stk.top();
        stk.pop();
        switch (op) {
          case '!':
            stk.push(f == 1 ? 't' : 'f');
            break;
          case '&':
            stk.push(f == 0 ? 't' : 'f');
            break;
          case '|':
            stk.push(t > 0 ? 't' : 'f');
            break;
          default:
            break;
        }
      }
    }
    return stk.top() == 't';
  }
};

int main() {
  assert(Solution().parseBoolExpr("!(f)"));
  assert(Solution().parseBoolExpr("|(f,t)"));
  assert(!Solution().parseBoolExpr("&(t,f)"));
  assert(!Solution().parseBoolExpr("|(&(t,f,t),!(t))"));
}
