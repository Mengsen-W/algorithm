/*
 * @Date: 2021-07-30 09:49:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-30 09:56:14
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  int titleToNumber(string s) {
    // 26进制转10进制
    int res = 0;
    for (char c : s) res = res * 26 + (c - 'A' + 1);
    return res;
  }
};

int main() {
  {
    string s = "A";
    assert(Solution{}.titleToNumber(s) == 1);
  }
  {
    string s = "AB";
    assert(Solution{}.titleToNumber(s) == 28);
  }
  {
    string s = "ZY";
    assert(Solution{}.titleToNumber(s) == 701);
  }

  return 0;
}