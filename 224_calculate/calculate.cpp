/*
 * @Date: 2021-03-10 08:09:25
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-10 08:33:37
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

int calculate(string s) {
  int res = 0;
  int i = 0;
  //将表达式分层，没有括号为敌0层，有一个括号为第1层……
  // sign 数组最后一个数用于表示当前层整体的符号，即层符
  //具体地，第0个数表示第0层（即没有括号时）的层符，应为 1
  //第1个数表示第一层的层符，如-(3+2-5),括号中3个数都为第一层，层符为 -1
  //数的真实符号为层符与层内符的叠加
  vector<int> sign;
  sign.push_back(1);
  //表示下一个数字真实的符号
  //等于数的层符与层内符的叠加
  int flag = 1;
  while (i < s.size()) {
    //读取数字
    if (isdigit(s[i])) {
      auto val = 0;
      while (i < s.size()) {
        if (s[i] == ' ')
          i++;
        else if (isdigit(s[i]))
          val = val * 10 + (s[i++] - '0');
        else
          break;
      }
      //添上符号，累计结果
      res += flag * val;
    }
    //运算符或空格
    else {
      switch (s[i]) {
        case '(':
          //再深入一层时，当前层符应变为当前符号flag
          sign.push_back(flag);
          break;
        case ')': {
          //跳出一层时，使用上一层的符号作为当前层符号
          sign.pop_back();
          break;
        }
        case '+': {
          //当前符号为层符叠加上层内符
          flag = sign.back();
          break;
        }
        case '-': {
          //叠加
          flag = -sign.back();
          break;
        }
      }
      i++;
    }
  }
  return res;
}

int main(void) {
  assert(calculate("1 + 1") == 2);
  assert(calculate("2 - 1 + 2") == 3);
  assert(calculate("(1 + (4 + 5 + 2) - 3) + (6 + 8)") == 23);
  return 0;
}