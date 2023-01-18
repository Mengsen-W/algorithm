/*
 * @Date: 2021-06-29 08:49:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-29 08:55:30
 */

#include <cassert>
#include <functional>
#include <string>
using namespace std;

string convertToTitle(int columnNumber) {
  string ans;
  while (columnNumber > 0) {
    --columnNumber;
    ans += columnNumber % 26 + 'A';
    columnNumber /= 26;
  }
  reverse(ans.begin(), ans.end());
  return ans;
}

int main() {
  assert(convertToTitle(1) == "A");
  assert(convertToTitle(28) == "AB");
  assert(convertToTitle(701) == "ZY");
}