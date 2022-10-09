/*
 * @Date: 2022-10-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-09
 * @FilePath: /algorithm/856_score_of_parentheses/score_of_parentheses.cpp
 */

#include <cassert>
#include <string>

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
  assert(Solution().scoreOfParentheses("()") == 1);
  assert(Solution().scoreOfParentheses("(())") == 2);
  assert(Solution().scoreOfParentheses("()()") == 2);
  assert(Solution().scoreOfParentheses("(()(()))") == 6);
}