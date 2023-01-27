/*
 * @Date: 2023-01-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-27
 * @FilePath: /algorithm/cpp/2309_greatest_letter/greatest_letter.cpp
 */

#include <cassert>
#include <string>

using namespace std;

class Solution {
 public:
  string greatestLetter(string s) {
    int lower = 0, upper = 0;
    for (char& c : s) {
      if (islower(c)) {
        lower |= 1 << (c - 'a');
      } else {
        upper |= 1 << (c - 'A');
      }
    }
    for (int i = 25; i >= 0; i--) {
      if (lower & upper & (1 << i)) {
        return string(1, 'A' + i);
      }
    }
    return "";
  }
};

int main() {
  {
    string s = "lEeTcOdE";
    string ans = "E";
    assert(Solution().greatestLetter(s) == ans);
  }

  {
    string s = "arRAzFif";
    string ans = "R";
    assert(Solution().greatestLetter(s) == ans);
  }

  {
    string s = "AbCdEfGhIjK";
    string ans = "";
    assert(Solution().greatestLetter(s) == ans);
  }
}