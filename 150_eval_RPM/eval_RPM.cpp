/*
 * @Date: 2021-03-20 09:03:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-20 09:07:22
 * @FilePath: \algorithm\150_eval_RPM\eval_RPM.cpp
 * @Description: file content
 */

#include <cassert>
#include <stack>
#include <string>
#include <vector>

using namespace std;

using namespace std;
inline bool is_number(string& token) {
  return !(token == "+" || token == "-" || token == "*" || token == "/");
}

int eval_RPN(vector<string> tokens) {
  stack<int> stk;
  int n = tokens.size();
  for (int i = 0; i < n; i++) {
    string& token = tokens[i];
    if (is_number(token)) {
      stk.push(atoi(token.c_str()));
    } else {
      int num = stk.top();
      stk.pop();
      switch (token[0]) {
        case '+':
          stk.top() += num;
          break;
        case '-':
          stk.top() -= num;
          break;
        case '*':
          stk.top() *= num;
          break;
        case '/':
          stk.top() /= num;
          break;
      }
    }
  }
  return stk.top();
}

int main() {
  assert(eval_RPN(vector<string>{"2", "1", "+", "3", "*"}) == 9);
  assert(eval_RPN(vector<string>{"4", "13", "5", "/", "+"}) == 6);
  assert(eval_RPN(vector<string>{"10", "6", "9", "3", "+", "-11", "*", "/", "*",
                                 "17", "+", "5", "+"}) == 22);
  return 0;
}