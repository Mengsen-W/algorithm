/*
 * @Date: 2022-05-28 10:30:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-28 10:36:14
 * @FilePath: /algorithm/1021_remove_outer_parentheses/remove_outer_parentheses.cpp
 */

#include <cassert>
#include <string>

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
  assert(Solution().removeOuterParentheses("(()())(())") == "()()()");
  assert(Solution().removeOuterParentheses("(()())(())(()(()))") == "()()()()(())");
  assert(Solution().removeOuterParentheses("()()") == "");

  return 0;
}
