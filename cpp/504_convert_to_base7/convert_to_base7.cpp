/*
 * @Date: 2022-03-07 00:02:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-07 00:02:52
 * @FilePath: /algorithm/504_convert_to_base7/convert_to_base7.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string convertToBase7(int num) {
    if (num == 0) {
      return "0";
    }
    bool negative = num < 0;
    num = abs(num);
    string digits;
    while (num > 0) {
      digits.push_back(num % 7 + '0');
      num /= 7;
    }
    if (negative) {
      digits.push_back('-');
    }
    reverse(digits.begin(), digits.end());
    return digits;
  }
};

int main() {
  assert(Solution().convertToBase7(100) == "202");
  assert(Solution().convertToBase7(-7) == "-10");
}