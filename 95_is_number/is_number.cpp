/*
 * @Date: 2021-06-17 07:34:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-17 07:41:59
 */

#include <cassert>
#include <string>
using namespace std;

bool isNumber(string s) {
  bool sign = 0, num = 0, dec = 0;
  int point = 0, exp = 0;

  int n = s.size();
  for (int i = 0; i < n; ++i) {
    if (s[i] == '+' || s[i] == '-') {
      //不能出现两个符号
      if (sign) return false;

      //如果符号位在最开始或者紧跟e后，是有效的符号位
      if (i == 0 || s[i - 1] == 'E' || s[i - 1] == 'e')
        sign = 1;
      else
        //符号不能出现在其他位置
        return false;

    } else if (s[i] == '.') {
      //点不能在e之后，并且只能存在一个点
      if (exp || point) return false;

      point = 1;
    } else if (s[i] == 'e' || s[i] == 'E') {
      //如果存在一个e或者e前的数不是一个有效的整数或者小数时，错误
      if (exp || (num == 0 && dec == 0)) return false;

      //否则就是有效的e，允许后面出现符号和整数
      //因此重置符号位，整数位，小数位状态，将e的标识符写为1
      else {
        sign = 0;
        num = 0;
        dec = 0;
        exp = 1;
      }
    } else if (s[i] >= '0' && s[i] <= '9') {
      //如果有小数点，当前数字就是小数位，小数位有效
      if (point) dec = 1;

      //否则就是整数位，整数位有效
      else
        num = 1;
    } else
      //其他字符直接返回
      return false;
  }
  //返回是否构成了有效的整数或者小数
  return num || dec;
}

int main() {
  {
    string s = "0";
    assert(isNumber(s));
  }
  {
    string s = "e";
    assert(!isNumber(s));
  }
  {
    string s = ".";
    assert(!isNumber(s));
  }
  {
    string s = ".1";
    assert(isNumber(s));
  }
  return 0;
}
