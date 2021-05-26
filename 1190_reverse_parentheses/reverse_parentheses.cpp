/*
 * @Date: 2021-05-26 09:53:20
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-26 10:53:10
 */

#include <cassert>
#include <stack>
#include <string>
#include <vector>

using namespace std;

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

int main() {
  assert(reverseParentheses("(abcd)") == "dcba");
  assert(reverseParentheses("(u(love)i)") == "iloveu");
  assert(reverseParentheses("(ed(et(oc))el)") == "leetcode");
  assert(reverseParentheses("a(bcdefghijkl(mno)p)q") == "apmnolkjihgfedcbq");
  return 0;
}