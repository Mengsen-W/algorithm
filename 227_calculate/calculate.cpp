/*
 * @Date: 2021-03-11 08:17:11
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-11 08:21:52
 */

#include <cassert>
#include <stack>
#include <string>

using namespace std;

int calculate(string s) {
  int res = 0, d = 0;
  char sign = '+';
  stack<int> nums;
  for (int i = 0; i < s.size(); ++i) {
    if (s[i] >= '0') {          //加减乘除和空格ASCII码都小于'0'
      d = d * 10 - '0' + s[i];  //进位(先减法)
    }
    if ((s[i] < '0' && s[i] != ' ') || i == s.size() - 1) {
      if (sign == '+') {
        nums.push(d);
      } else if (sign == '-') {
        nums.push(-d);
      } else if (sign == '*' || sign == '/') {
        int tmp = sign == '*' ? nums.top() * d : nums.top() / d;
        nums.pop();
        nums.push(tmp);
      }
      sign = s[i];  //保存当前符号
      d = 0;
    }
  }
  for (; !nums.empty(); nums.pop()) {
    res += nums.top();
  }
  return res;
}

int main(void) {
  assert(calculate("3+2*2") == 7);
  assert(calculate("3/2") == 1);
  assert(calculate("3+5 / 2") == 5);
  return 0;
}