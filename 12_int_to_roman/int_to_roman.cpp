/*
 * @Date: 2021-05-14 08:44:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-14 09:05:08
 */

#include <cassert>
#include <string>
using namespace std;

const string thousands[] = {"", "M", "MM", "MMM"};
const string hundreds[] = {"",  "C",  "CC",  "CCC",  "CD",
                           "D", "DC", "DCC", "DCCC", "CM"};
const string tens[] = {"",  "X",  "XX",  "XXX",  "XL",
                       "L", "LX", "LXX", "LXXX", "XC"};
const string ones[] = {"",  "I",  "II",  "III",  "IV",
                       "V", "VI", "VII", "VIII", "IX"};

string intToRoman(int num) {
  return thousands[num / 1000] + hundreds[num % 1000 / 100] +
         tens[num % 100 / 10] + ones[num % 10];
}

int main() {
  assert(intToRoman(3) == "III");
  assert(intToRoman(4) == "IV");
  assert(intToRoman(9) == "IX");
  assert(intToRoman(58) == "LVIII");
  assert(intToRoman(1994) == "MCMXCIV");
  return 0;
}
