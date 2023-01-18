/*
 * @Date: 2022-10-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-01
 * @FilePath: /algorithm/1694_reformat_number/reformat_number.cpp
 */

#include <assert.h>

#include <cassert>
#include <iostream>
#include <string>

using namespace std;

class Solution {
 public:
  string reformatNumber(string number) {
    string digits;
    for (char ch : number) {
      if (isdigit(ch)) {
        digits.push_back(ch);
      }
    }

    int n = digits.size();
    int pt = 0;
    string ans;
    while (n) {
      if (n > 4) {
        ans += digits.substr(pt, 3) + "-";
        pt += 3;
        n -= 3;
      } else {
        if (n == 4) {
          ans += digits.substr(pt, 2) + "-" + digits.substr(pt + 2, 2);
        } else {
          ans += digits.substr(pt, n);
        }
        break;
      }
    }
    return ans;
  }
};

int main() {
  assert(Solution().reformatNumber("1-23-45 6") == "123-456");
  assert(Solution().reformatNumber("123 4-567") == "123-45-67");
  assert(Solution().reformatNumber("123 4-5678") == "123-456-78");
  assert(Solution().reformatNumber("12") == "12");
  assert(Solution().reformatNumber("--17-5 229 35-39475 ") == "175-229-353-94-75");
}